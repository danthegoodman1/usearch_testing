# usearch_testing

https://github.com/unum-cloud/usearch/tree/main/rust

```
=== Distributed Search (With Exact Reranking) ===

Loaded 10 test vectors
  • Loading test vectors took 12.5µs
Created view of shard 0 with 3333 vectors (memory usage: 29736 bytes)
Created view of shard 1 with 3333 vectors (memory usage: 29736 bytes)
Created view of shard 2 with 3333 vectors (memory usage: 29736 bytes)
  • Opening shard views took 215.792µs

Query vector 0:
  Collected 60 candidates from all shards
    • Candidate collection took 456.583µs
    • Building rerank index took 369.417µs
    • Exact rerank search took 2µs
  Exact reranking top-5 results:
    1: key=4182 distance=-36.723827 (from shard 1)
    2: key=7587 distance=-36.642307 (from shard 2)
    3: key=2602 distance=-35.987671 (from shard 0)
    4: key=8733 distance=-35.923222 (from shard 2)
    5: key=4504 distance=-35.663845 (from shard 1)

Query vector 1:
  Collected 60 candidates from all shards
    • Candidate collection took 157.291µs
    • Building rerank index took 354.75µs
    • Exact rerank search took 1.75µs
  Exact reranking top-5 results:
    1: key=20 distance=-36.966377 (from shard 0)
    2: key=7587 distance=-36.529949 (from shard 2)
    3: key=4858 distance=-36.487251 (from shard 1)
    4: key=1940 distance=-36.207767 (from shard 0)
    5: key=2518 distance=-36.185081 (from shard 0)

Query vector 2:
  Collected 60 candidates from all shards
    • Candidate collection took 152.833µs
    • Building rerank index took 369.542µs
    • Exact rerank search took 2µs
  Exact reranking top-5 results:
    1: key=7587 distance=-39.470642 (from shard 2)
    2: key=1734 distance=-39.343460 (from shard 0)
    3: key=8234 distance=-39.114182 (from shard 2)
    4: key=4633 distance=-39.101620 (from shard 1)
    5: key=8266 distance=-38.916084 (from shard 2)

Query vector 3:
  Collected 60 candidates from all shards
    • Candidate collection took 154.833µs
    • Building rerank index took 359.083µs
    • Exact rerank search took 1.917µs
  Exact reranking top-5 results:
    1: key=7587 distance=-38.425995 (from shard 2)
    2: key=7915 distance=-37.478065 (from shard 2)
    3: key=8884 distance=-37.230236 (from shard 2)
    4: key=6555 distance=-36.956955 (from shard 1)
    5: key=3934 distance=-36.894711 (from shard 1)

Query vector 4:
  Collected 60 candidates from all shards
    • Candidate collection took 143.875µs
    • Building rerank index took 352.167µs
    • Exact rerank search took 1.709µs
  Exact reranking top-5 results:
    1: key=8234 distance=-38.257912 (from shard 2)
    2: key=6051 distance=-38.155411 (from shard 1)
    3: key=6481 distance=-37.265381 (from shard 1)
    4: key=5016 distance=-36.920650 (from shard 1)
    5: key=2006 distance=-36.850101 (from shard 0)

Query vector 5:
  Collected 60 candidates from all shards
    • Candidate collection took 135.292µs
    • Building rerank index took 355.708µs
    • Exact rerank search took 1.917µs
  Exact reranking top-5 results:
    1: key=8266 distance=-37.163532 (from shard 2)
    2: key=6051 distance=-36.775517 (from shard 1)
    3: key=2982 distance=-36.773186 (from shard 0)
    4: key=8733 distance=-36.658104 (from shard 2)
    5: key=5016 distance=-36.335869 (from shard 1)

Query vector 6:
  Collected 60 candidates from all shards
    • Candidate collection took 138.833µs
    • Building rerank index took 357.708µs
    • Exact rerank search took 1.834µs
  Exact reranking top-5 results:
    1: key=4272 distance=-38.716942 (from shard 1)
    2: key=8435 distance=-38.716911 (from shard 2)
    3: key=3691 distance=-38.685787 (from shard 1)
    4: key=6555 distance=-38.412781 (from shard 1)
    5: key=7455 distance=-38.366539 (from shard 2)

Query vector 7:
  Collected 60 candidates from all shards
    • Candidate collection took 134.083µs
    • Building rerank index took 356.583µs
    • Exact rerank search took 1.791µs
  Exact reranking top-5 results:
    1: key=5016 distance=-34.548325 (from shard 1)
    2: key=8349 distance=-33.648853 (from shard 2)
    3: key=5212 distance=-32.994324 (from shard 1)
    4: key=2414 distance=-32.924534 (from shard 0)
    5: key=8733 distance=-32.800755 (from shard 2)

Query vector 8:
  Collected 60 candidates from all shards
    • Candidate collection took 138.5µs
    • Building rerank index took 362µs
    • Exact rerank search took 1.75µs
  Exact reranking top-5 results:
    1: key=6555 distance=-37.491554 (from shard 1)
    2: key=7587 distance=-37.336205 (from shard 2)
    3: key=5212 distance=-37.295551 (from shard 1)
    4: key=6632 distance=-37.016273 (from shard 1)
    5: key=4633 distance=-36.979610 (from shard 1)

Query vector 9:
  Collected 60 candidates from all shards
    • Candidate collection took 138.041µs
    • Building rerank index took 365.083µs
    • Exact rerank search took 1.75µs
  Exact reranking top-5 results:
    1: key=6555 distance=-40.274170 (from shard 1)
    2: key=5313 distance=-40.019382 (from shard 1)
    3: key=9558 distance=-39.832466 (from shard 2)
    4: key=3615 distance=-39.689705 (from shard 1)
    5: key=4272 distance=-39.516075 (from shard 1)
```
