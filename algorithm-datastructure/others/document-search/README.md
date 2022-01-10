# Points to consider

## Robustness
- Raise an error when no arguments given
- Run without problem when given more than 2 arguments
- Run with Japanese argument
- Display error message when an empty strint/whitespace is given
- Raise an error when a huge input is passed

## Great UX
- Is a use able to know what to do next when he/she encountered an error
- Search even in title for keyword
- How to increase accuracy of hits
- Design for the case where a given keyword is partly matches to some words in article

## Performance
- How to improve performance
  - need binary search? merge sort?

## Readability


## Maintainability


# Going futher

## About text search engine

- [Meillisearch](https://github.com/meilisearch/MeiliSearch): blazing fast search database written in Rust
  - uses [milli](https://github.com/meilisearch/milli): A single index manager
    - which uses [heed](https://github.com/Kerollmops/heed): LMDB/MDBX wrapper
      - LMDB(Lightning Memory-Mapped Database)
        - [Wikipedia](https://en.wikipedia.org/wiki/Lightning_Memory-Mapped_Database)
        - [Youtube](https://www.youtube.com/watch?v=Rx1-in-a1Xc)

- LMDB
  - Mapped Memory, thus no need to use malloc/memcpy on read
    - A memory-mapped file is a segment of virtual memory that has been assigned a direct byte-for-byte correlation with some portion of a file or file-like resource(wiki)
    - thus fast
  - Mapped Memory is a file, .mdb file represents it
    - https://blog.separateconcerns.com/2016-04-03-lmdb-format.html
    - https://en.wikipedia.org/wiki/Memory-mapped_file
    - https://www.youtube.com/watch?v=m7E9piHcfr4

- LMDB holds data in a form of [B+tree](https://en.wikipedia.org/wiki/B%2B_tree)
  - B+tree holds multi-layer index, each node contains child node and and leaf node have record pointer
  - leaf node construct linked list
  - https://www.youtube.com/watch?v=Rx1-in-a1Xc


- Rust
  - FFI(Foreign Function Interface)
    - [link()] format
    - this tells liker to link functions(object file codes)
- Linker
  - [what is linker](https://www.youtube.com/watch?v=N2y6csonII4)


# Benchmark
➜  rust-inverted-index git:(master) hyperfine "target/release/inverted_index the"  
Benchmark #1: target/release/inverted_index the
  Time (mean ± σ):       6.4 ms ±   1.2 ms    [User: 1.7 ms, System: 2.1 ms]
  Range (min … max):     4.3 ms …  14.4 ms    138 runs
