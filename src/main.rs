use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;
use std::io::{Read, Write};
use std::time::Instant;
use usearch::{new_index, Index, IndexOptions, MetricKind, ScalarKind};

const DIMS: usize = 128;
const SHARDS: usize = 3;
const VECTORS_PER_SHARD: usize = 3333;
const TEST_VECTORS_COUNT: usize = 10;
const TOP_K: usize = 5;

fn create_index_options(dims: usize) -> IndexOptions {
    IndexOptions {
        dimensions: dims,
        metric: MetricKind::IP,
        quantization: ScalarKind::F32,
        multi: false,
        connectivity: 0,
        expansion_add: 0,
        expansion_search: 0,
    }
}

fn generate_random_vectors(count: usize, dims: usize, seed: u64) -> Vec<Vec<f32>> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vectors = Vec::with_capacity(count);

    for _ in 0..count {
        let mut vector = vec![0.0f32; dims];
        for value in &mut vector {
            *value = rng.random();
        }
        vectors.push(vector);
    }

    vectors
}

fn build_shards() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Building {} shards with {} vectors each...",
        SHARDS, VECTORS_PER_SHARD
    );

    let options = create_index_options(DIMS);

    for shard_id in 0..SHARDS {
        let index: Index = new_index(&options)?;
        index.reserve(VECTORS_PER_SHARD)?;

        // Generate vectors with different seeds for each shard
        let vectors = generate_random_vectors(VECTORS_PER_SHARD, DIMS, (shard_id * 1000) as u64);

        // Add vectors to this shard
        let base_key = shard_id * VECTORS_PER_SHARD;
        for (i, vector) in vectors.iter().enumerate() {
            let key = (base_key + i) as u64;
            index.add(key, vector)?;
        }

        // Save shard to disk
        let shard_path = format!("./shard_{}.index", shard_id);
        index.save(&shard_path)?;
        println!(
            "  Shard {} saved with {} vectors to {}",
            shard_id,
            index.size(),
            shard_path
        );
    }

    // Generate and save test vectors
    println!("\nGenerating {} test vectors...", TEST_VECTORS_COUNT);
    let test_vectors = generate_random_vectors(TEST_VECTORS_COUNT, DIMS, 99999);

    // Save test vectors as binary file
    let mut file = fs::File::create("test_vectors.bin")?;
    for vector in &test_vectors {
        for value in vector {
            file.write_all(&value.to_ne_bytes())?;
        }
    }
    println!("Test vectors saved to test_vectors.bin");

    Ok(())
}

fn load_test_vectors() -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("test_vectors.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let float_count = buffer.len() / 4;
    let mut floats = Vec::with_capacity(float_count);

    for chunk in buffer.chunks_exact(4) {
        let bytes: [u8; 4] = chunk.try_into().unwrap();
        floats.push(f32::from_ne_bytes(bytes));
    }

    let mut vectors = Vec::with_capacity(TEST_VECTORS_COUNT);
    for chunk in floats.chunks_exact(DIMS) {
        vectors.push(chunk.to_vec());
    }

    Ok(vectors)
}

fn compare_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Memory Usage Comparison: Load vs View ===\n");

    let options = create_index_options(DIMS);
    let test_vectors = load_test_vectors()?;
    let query = &test_vectors[0];

    // Test 1: Using load (loads entire index into memory)
    println!("1. Loading index into memory:");
    let index_loaded: Index = new_index(&options)?;
    index_loaded.load("./shard_0.index")?;
    println!("   Index loaded, size: {}", index_loaded.size());
    println!("   Memory usage: {} bytes", index_loaded.memory_usage());

    let matches = index_loaded.search(query, TOP_K)?;
    println!("   Search completed, found {} results", matches.keys.len());

    // Test 2: Using view (memory-mapped, doesn't load into RAM)
    println!("\n2. Using memory-mapped view:");
    let index_view: Index = new_index(&options)?;
    index_view.view("./shard_0.index")?;
    println!("   Index view created, size: {}", index_view.size());
    println!(
        "   Memory usage: {} bytes (much lower!)",
        index_view.memory_usage()
    );

    let matches_view = index_view.search(query, TOP_K)?;
    println!(
        "   Search completed, found {} results",
        matches_view.keys.len()
    );

    // Verify results are the same
    assert_eq!(
        matches.keys, matches_view.keys,
        "Results should be identical"
    );
    println!("\n   ✓ Results are identical between load and view methods");

    Ok(())
}

fn distributed_search_with_reranking() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Distributed Search (With Exact Reranking) ===\n");

    const CANDIDATES_PER_SHARD: usize = 20; // Get more candidates for reranking

    // Load test vectors
    let load_vectors_start = Instant::now();
    let test_vectors = load_test_vectors()?;
    let load_vectors_duration = load_vectors_start.elapsed();
    println!("Loaded {} test vectors", test_vectors.len());
    println!("  • Loading test vectors took {:?}", load_vectors_duration);

    // Create memory-mapped views of all shards (lower memory usage!)
    let mut shards = Vec::new();
    let options = create_index_options(DIMS);
    let open_views_start = Instant::now();

    for shard_id in 0..SHARDS {
        let index: Index = new_index(&options)?;
        let shard_path = format!("./shard_{}.index", shard_id);
        index.view(&shard_path)?; // Using view instead of load!
        println!(
            "Created view of shard {} with {} vectors (memory usage: {} bytes)",
            shard_id,
            index.size(),
            index.memory_usage()
        );
        shards.push(index);
    }
    println!(
        "  • Opening shard views took {:?}",
        open_views_start.elapsed()
    );

    // For each test vector
    for (test_id, query) in test_vectors.iter().enumerate() {
        println!("\nQuery vector {}:", test_id);

        // Step 1: Collect candidate keys from all shards
        let collect_candidates_start = Instant::now();
        let mut candidate_keys = Vec::new();

        for (shard_id, shard) in shards.iter().enumerate() {
            let matches = shard.search(query, CANDIDATES_PER_SHARD)?;
            for key in matches.keys.iter() {
                candidate_keys.push((*key, shard_id));
            }
        }

        println!(
            "  Collected {} candidates from all shards",
            candidate_keys.len()
        );
        println!(
            "    • Candidate collection took {:?}",
            collect_candidates_start.elapsed()
        );

        // Step 2: Build a small in-memory index with just the candidates
        let build_rerank_start = Instant::now();
        let rerank_index: Index = new_index(&options)?;
        rerank_index.reserve(candidate_keys.len())?;

        for (key, shard_id) in &candidate_keys {
            // Retrieve the actual vector from the appropriate shard
            let mut vector = vec![0.0f32; DIMS];
            shards[*shard_id].export(*key, &mut vector)?;
            rerank_index.add(*key, &vector)?;
        }
        println!(
            "    • Building rerank index took {:?}",
            build_rerank_start.elapsed()
        );

        // Step 3: Do exact search in the reranking index
        // Since the index is small, this will be nearly exhaustive
        let exact_search_start = Instant::now();
        let final_matches = rerank_index.exact_search(query, TOP_K)?;
        println!(
            "    • Exact rerank search took {:?}",
            exact_search_start.elapsed()
        );

        println!("  Exact reranking top-{} results:", TOP_K);
        for (rank, (key, distance)) in final_matches
            .keys
            .iter()
            .zip(&final_matches.distances)
            .enumerate()
        {
            // Find which shard this key came from
            let shard_id = candidate_keys
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, s)| s)
                .unwrap();

            println!(
                "    {}: key={} distance={:.6} (from shard {})",
                rank + 1,
                key,
                distance,
                shard_id
            );
        }
    }

    Ok(())
}

fn main() {
    // Build shards and save test vectors
    if let Err(e) = build_shards() {
        eprintln!("Error building shards: {}", e);
        return;
    }

    // Compare memory usage between load and view
    if let Err(e) = compare_memory_usage() {
        eprintln!("Error comparing memory usage: {}", e);
        return;
    }

    // Demonstrate exact reranking approach
    if let Err(e) = distributed_search_with_reranking() {
        eprintln!("Error during distributed search with reranking: {}", e);
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_building() {
        build_shards().expect("Should build shards successfully");

        // Verify shard files exist
        for shard_id in 0..SHARDS {
            let shard_path = format!("./shard_{}.index", shard_id);
            assert!(std::path::Path::new(&shard_path).exists());
        }

        // Verify test vectors file exists
        assert!(std::path::Path::new("test_vectors.bin").exists());
    }

    #[test]
    fn test_distributed_search() {
        // Ensure shards are built first
        if !std::path::Path::new("shard_0.index").exists() {
            build_shards().expect("Should build shards");
        }

        distributed_search_with_reranking()
            .expect("Should perform distributed search with reranking successfully");
    }
}
