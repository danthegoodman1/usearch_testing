# usearch_testing

https://github.com/unum-cloud/usearch/tree/main/rust

```
=== Distributed Search (With Exact Reranking) ===

Loaded 10 test vectors
  • Loading test vectors took 33.959µs
Created view of shard 0 with 1000000 vectors (memory usage: 8003072 bytes)
  • Shard 0 view creation took 29.831ms
Created view of shard 1 with 1000000 vectors (memory usage: 8003072 bytes)
  • Shard 1 view creation took 32.243167ms
Created view of shard 2 with 1000000 vectors (memory usage: 8003072 bytes)
  • Shard 2 view creation took 30.18775ms
  • Opening shard views took 92.296292ms

Query vector 0:
  Collected 60 candidates from all shards
    • Candidate collection took 8.098042ms
    • Building rerank index took 299.875µs
    • Exact rerank search took 2.292µs
  Exact reranking top-5 results:
    1: key=2630527 distance=-38.487709 (from shard 2)
    2: key=769720 distance=-38.425270 (from shard 0)
    3: key=875971 distance=-37.939098 (from shard 0)
    4: key=1818337 distance=-37.858330 (from shard 1)
    5: key=2422974 distance=-37.785500 (from shard 2)
  • Ground-truth exact + merge took 163.478875ms
    • Shard 0 exact_search(100) took 53.615ms
    • Shard 1 exact_search(100) took 55.01775ms
    • Shard 2 exact_search(100) took 54.8385ms
  Recall@5 vs exact global: 1.000

Query vector 1:
  Collected 60 candidates from all shards
    • Candidate collection took 1.804875ms
    • Building rerank index took 315.125µs
    • Exact rerank search took 2.458µs
  Exact reranking top-5 results:
    1: key=2469668 distance=-39.027069 (from shard 2)
    2: key=2630527 distance=-38.927311 (from shard 2)
    3: key=2986511 distance=-38.696934 (from shard 2)
    4: key=1080120 distance=-38.562710 (from shard 1)
    5: key=620632 distance=-38.413147 (from shard 0)
  • Ground-truth exact + merge took 92.219667ms
    • Shard 0 exact_search(100) took 30.633083ms
    • Shard 1 exact_search(100) took 30.943792ms
    • Shard 2 exact_search(100) took 30.635791ms
  Recall@5 vs exact global: 1.000

Query vector 2:
  Collected 60 candidates from all shards
    • Candidate collection took 1.789125ms
    • Building rerank index took 293.792µs
    • Exact rerank search took 2.458µs
  Exact reranking top-5 results:
    1: key=1080120 distance=-41.664375 (from shard 1)
    2: key=2947400 distance=-41.434456 (from shard 2)
    3: key=1088171 distance=-41.148102 (from shard 1)
    4: key=254849 distance=-41.024860 (from shard 0)
    5: key=1738941 distance=-41.001816 (from shard 1)
  • Ground-truth exact + merge took 92.295125ms
    • Shard 0 exact_search(100) took 30.718334ms
    • Shard 1 exact_search(100) took 30.811709ms
    • Shard 2 exact_search(100) took 30.75875ms
  Recall@5 vs exact global: 1.000

Query vector 3:
  Collected 60 candidates from all shards
    • Candidate collection took 1.804625ms
    • Building rerank index took 293.208µs
    • Exact rerank search took 2.542µs
  Exact reranking top-5 results:
    1: key=1639566 distance=-40.011330 (from shard 1)
    2: key=746435 distance=-39.885635 (from shard 0)
    3: key=2158668 distance=-39.701622 (from shard 2)
    4: key=2827240 distance=-39.623573 (from shard 2)
    5: key=2422974 distance=-39.534302 (from shard 2)
  • Ground-truth exact + merge took 91.286791ms
    • Shard 0 exact_search(100) took 30.537083ms
    • Shard 1 exact_search(100) took 30.393625ms
    • Shard 2 exact_search(100) took 30.349542ms
  Recall@5 vs exact global: 1.000

Query vector 4:
  Collected 60 candidates from all shards
    • Candidate collection took 1.753458ms
    • Building rerank index took 298.416µs
    • Exact rerank search took 2.292µs
  Exact reranking top-5 results:
    1: key=2887975 distance=-40.472237 (from shard 2)
    2: key=234577 distance=-39.453110 (from shard 0)
    3: key=2827240 distance=-39.451736 (from shard 2)
    4: key=612922 distance=-39.243252 (from shard 0)
    5: key=282984 distance=-39.222237 (from shard 0)
  • Ground-truth exact + merge took 91.468916ms
    • Shard 0 exact_search(100) took 30.688916ms
    • Shard 1 exact_search(100) took 30.3015ms
    • Shard 2 exact_search(100) took 30.455ms
  Recall@5 vs exact global: 1.000

Query vector 5:
  Collected 60 candidates from all shards
    • Candidate collection took 1.864708ms
    • Building rerank index took 297.291µs
    • Exact rerank search took 5.208µs
  Exact reranking top-5 results:
    1: key=1818337 distance=-39.707321 (from shard 1)
    2: key=1129714 distance=-38.799534 (from shard 1)
    3: key=234577 distance=-38.739075 (from shard 0)
    4: key=783830 distance=-38.727921 (from shard 0)
    5: key=1598392 distance=-38.634331 (from shard 1)
  • Ground-truth exact + merge took 92.695584ms
    • Shard 0 exact_search(100) took 30.318792ms
    • Shard 1 exact_search(100) took 30.457208ms
    • Shard 2 exact_search(100) took 31.9025ms
  Recall@5 vs exact global: 1.000

Query vector 6:
  Collected 60 candidates from all shards
    • Candidate collection took 1.78825ms
    • Building rerank index took 324.875µs
    • Exact rerank search took 4.959µs
  Exact reranking top-5 results:
    1: key=620632 distance=-41.558220 (from shard 0)
    2: key=234577 distance=-41.352226 (from shard 0)
    3: key=805858 distance=-41.347534 (from shard 0)
    4: key=2991360 distance=-41.157536 (from shard 2)
    5: key=746435 distance=-41.139072 (from shard 0)
  • Ground-truth exact + merge took 92.914375ms
    • Shard 0 exact_search(100) took 30.511167ms
    • Shard 1 exact_search(100) took 31.016125ms
    • Shard 2 exact_search(100) took 31.340959ms
  Recall@5 vs exact global: 1.000

Query vector 7:
  Collected 60 candidates from all shards
    • Candidate collection took 1.924417ms
    • Building rerank index took 378.375µs
    • Exact rerank search took 7.833µs
  Exact reranking top-5 results:
    1: key=2947400 distance=-35.898407 (from shard 2)
    2: key=2056538 distance=-35.475960 (from shard 2)
    3: key=237414 distance=-35.465527 (from shard 0)
    4: key=859596 distance=-35.319340 (from shard 0)
    5: key=612922 distance=-35.297379 (from shard 0)
  • Ground-truth exact + merge took 95.156292ms
    • Shard 0 exact_search(100) took 31.821667ms
    • Shard 1 exact_search(100) took 31.73425ms
    • Shard 2 exact_search(100) took 31.590208ms
  Recall@5 vs exact global: 1.000

Query vector 8:
  Collected 60 candidates from all shards
    • Candidate collection took 1.752208ms
    • Building rerank index took 330.125µs
    • Exact rerank search took 2.625µs
  Exact reranking top-5 results:
    1: key=2818912 distance=-39.398262 (from shard 2)
    2: key=2630527 distance=-39.314247 (from shard 2)
    3: key=780611 distance=-39.074844 (from shard 0)
    4: key=1818337 distance=-39.069729 (from shard 1)
    5: key=357871 distance=-39.008282 (from shard 0)
  • Ground-truth exact + merge took 95.033667ms
    • Shard 0 exact_search(100) took 31.479667ms
    • Shard 1 exact_search(100) took 32.315417ms
    • Shard 2 exact_search(100) took 31.206792ms
  Recall@5 vs exact global: 1.000

Query vector 9:
  Collected 60 candidates from all shards
    • Candidate collection took 2.19425ms
    • Building rerank index took 350.166µs
    • Exact rerank search took 5.708µs
  Exact reranking top-5 results:
    1: key=2986511 distance=-42.028530 (from shard 2)
    2: key=234577 distance=-42.015015 (from shard 0)
    3: key=2858630 distance=-41.819557 (from shard 2)
    4: key=2709913 distance=-41.540531 (from shard 2)
    5: key=237414 distance=-41.531784 (from shard 0)
  • Ground-truth exact + merge took 94.026541ms
    • Shard 0 exact_search(100) took 30.758167ms
    • Shard 1 exact_search(100) took 31.061834ms
    • Shard 2 exact_search(100) took 32.194333ms
  Recall@5 vs exact global: 1.000

Average Recall@5 across 10 queries: 1.000
```

```
-rw-r--r--  1 dangoodman  staff   630M Sep 24 16:37 shard_2.index
-rw-r--r--  1 dangoodman  staff   630M Sep 24 16:37 shard_1.index
-rw-r--r--  1 dangoodman  staff   630M Sep 24 16:37 shard_0.index
```
