# Source

## Introduction

We use the `source` project as a static JSON response for the benchmarks with HTTP caching headers. We use a static response to eliminate the factor of the source server causing overhead to the benchmark. We use this server for all the benchmarks to consider the benchmark results fair for all implementations.

## Usage

```bash
cargo run --release <test_type: big|medium|small>
```
