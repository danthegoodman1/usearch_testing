use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;
use std::io::{Read, Write};
use std::time::Instant;
use std::collections::HashSet;
use usearch::{Index, IndexOptions, MetricKind, ScalarKind, new_index};

const DIMS: usize = 128;
const SHARDS: usize = 3;
const VECTORS_PER_SHARD: usize = 1_000_000;
const TEST_VECTORS_COUNT: usize = 10;
const TOP_K: usize = 5;
// Size of exact results taken from each shard to build global ground truth
const EXACT_GT_PER_SHARD: usize = 100;

fn create_index_options(dims: usize) -> IndexOptions {
    IndexOptions {
        dimensions: dims,
        metric: MetricKind::IP,
        quantization: ScalarKind::F32,
        multi: false,
        connectivity: 24,
        expansion_add: 200,
        expansion_search: 80,
    }
}

fn create_flat_index_options(dims: usize) -> IndexOptions {
    IndexOptions {
        dimensions: dims,
        metric: MetricKind::IP,
        quantization: ScalarKind::F32,
        multi: false,
        connectivity: 0,    // No graph structure - flat/linear index
        expansion_add: 0,   // No complex building process
        expansion_search: 0, // Brute force search (perfect for small sets)
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

    let total_start = Instant::now();
    let options = create_index_options(DIMS);

    for shard_id in 0..SHARDS {
        let shard_start = Instant::now();
        println!("  Building shard {}...", shard_id);

        let index_creation_start = Instant::now();
        let index: Index = new_index(&options)?;
        index.reserve(VECTORS_PER_SHARD)?;
        println!("    • Index creation: {:?}", index_creation_start.elapsed());

        // Generate vectors with different seeds for each shard
        let vector_gen_start = Instant::now();
        let vectors = generate_random_vectors(VECTORS_PER_SHARD, DIMS, (shard_id * 1000) as u64);
        println!("    • Vector generation: {:?}", vector_gen_start.elapsed());

        // Add vectors to this shard
        let vector_add_start = Instant::now();
        let base_key = shard_id * VECTORS_PER_SHARD;
        for (i, vector) in vectors.iter().enumerate() {
            let key = (base_key + i) as u64;
            index.add(key, vector)?;
        }
        println!(
            "    • Adding {} vectors: {:?}",
            VECTORS_PER_SHARD,
            vector_add_start.elapsed()
        );

        // Save shard to disk
        let save_start = Instant::now();
        let shard_path = format!("./shard_{}.index", shard_id);
        index.save(&shard_path)?;
        println!("    • Saving to disk: {:?}", save_start.elapsed());

        println!(
            "  Shard {} completed in {:?} with {} vectors saved to {}",
            shard_id,
            shard_start.elapsed(),
            index.size(),
            shard_path
        );
    }

    println!("Total shard building time: {:?}", total_start.elapsed());

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
        let shard_view_start = Instant::now();
        let index: Index = new_index(&options)?;
        let shard_path = format!("./shard_{}.index", shard_id);
        index.view(&shard_path)?; // Using view instead of load!
        let shard_view_duration = shard_view_start.elapsed();
        println!(
            "Created view of shard {} with {} vectors (memory usage: {} bytes)",
            shard_id,
            index.size(),
            index.memory_usage()
        );
        println!(
            "  • Shard {} view creation took {:?}",
            shard_id, shard_view_duration
        );
        shards.push(index);
    }
    println!(
        "  • Opening shard views took {:?}",
        open_views_start.elapsed()
    );

    // For each test vector
    let mut recall_sum = 0.0f64;
    let mut num_queries = 0usize;
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
        // Use flat/linear index for reranking - no need for HNSW overhead on 60 vectors!
        let build_rerank_start = Instant::now();
        let flat_options = create_flat_index_options(DIMS);
        let rerank_index: Index = new_index(&flat_options)?;
        rerank_index.reserve(candidate_keys.len())?;

        for (key, shard_id) in &candidate_keys {
            // Retrieve the actual vector from the appropriate shard
            let mut vector = vec![0.0f32; DIMS];
            shards[*shard_id].get(*key, &mut vector)?;
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

        // Compute ground-truth global top-K by exact per-shard + merge (timed)
        let gt_total_start = Instant::now();
        let mut all_exact: Vec<(u64, f32)> = Vec::with_capacity(SHARDS * EXACT_GT_PER_SHARD);
        let mut per_shard_exact_stats: Vec<(usize, std::time::Duration)> = Vec::with_capacity(SHARDS);
        for shard in shards.iter() {
            let per_shard_k = EXACT_GT_PER_SHARD.min(shard.size());
            let shard_exact_start = Instant::now();
            let exact = shard.exact_search(query, per_shard_k)?;
            let shard_exact_elapsed = shard_exact_start.elapsed();
            per_shard_exact_stats.push((per_shard_k, shard_exact_elapsed));
            for (k, d) in exact.keys.iter().zip(exact.distances.iter()) {
                all_exact.push((*k, *d));
            }
        }
        // Distances: lower is better across metrics (IP returns negative distances)
        all_exact.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        all_exact.truncate(TOP_K);

        println!("  • Ground-truth exact + merge took {:?}", gt_total_start.elapsed());
        for (i, (k, dur)) in per_shard_exact_stats.iter().enumerate() {
            println!("    • Shard {} exact_search({}) took {:?}", i, k, dur);
        }

        let gt_keys: HashSet<u64> = all_exact.iter().map(|(k, _)| *k).collect();
        let pred_keys: HashSet<u64> = final_matches.keys.iter().cloned().collect();
        let intersect = gt_keys.intersection(&pred_keys).count();
        let recall = intersect as f64 / TOP_K as f64;
        println!("  Recall@{} vs exact global: {:.3}", TOP_K, recall);

        recall_sum += recall;
        num_queries += 1;
    }

    if num_queries > 0 {
        println!(
            "\nAverage Recall@{} across {} queries: {:.3}",
            TOP_K,
            num_queries,
            recall_sum / num_queries as f64
        );
    }

    Ok(())
}

fn main() {
    // // Build shards and save test vectors
    // if let Err(e) = build_shards() {
    //     eprintln!("Error building shards: {}", e);
    //     return;
    // }

    // // Compare memory usage between load and view
    // if let Err(e) = compare_memory_usage() {
    //     eprintln!("Error comparing memory usage: {}", e);
    //     return;
    // }

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
