# Source

## Introduction

This is the data source server. It serves a static json response for the benchmarks with http caching headers. We use a static response to eliminate the factor of the source server providing overhead to the benchmark. So the benchmarks can be considered as fair ground for all implementations.

## Usage

```bash
cargo run --release <test_type: big|medium|small>
```
