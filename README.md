# Federation Benchmarks

Explore and compare the performance of the fastest GraphQL federation routers through our comprehensive benchmarks.

- [Introduction](#introduction)
- [Benchmark Results](#benchmark-results)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Resources](#resources)
- [Contribute](#contribute)

## Introduction

This document presents a comparative analysis of several renowned GraphQL Federation routers. Dive deep into the performance metrics and get insights regarding their throughput and latency.

> **NOTE:** This is a work-in-progress suite of benchmarks, and we would appreciate help from the community to add more routers or tune the existing ones for better performance.

## Benchmark Results

### Requests Per Second

<!-- PERFORMANCE_RESULTS_START -->
| Server | [112,838 bytes](./source/big.json)| [12598 bytes](./source/medium.json)| [362 bytes](./source/small.json) |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | `2,415 RPS` | `4,759 RPS` | `4,779 RPS` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | `1,651 RPS` | `4,596 RPS` | `7,874 RPS` |
| [Grafbase](https://github.com/grafbase/grafbase) | `1,981 RPS` | `3,660 RPS` | `4,536 RPS` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | `1,040 RPS` | `1,786 RPS` | `3,132 RPS` |
| [Apollo](https://github.com/apollographql/router) | `1,213 RPS` | `2,254 RPS` | `2,392 RPS` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | `8,020 RPS` | `12,659 RPS` | `16,052 RPS` |
| [Grafbase](https://github.com/grafbase/grafbase) | `2,212 RPS` | `5,101 RPS` | `7,415 RPS` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | `1,559 RPS` | `3,382 RPS` | `5,718 RPS` |
<!-- PERFORMANCE_RESULTS_END -->

### Latency

<!-- LATENCY_RESULTS_START -->
TODO: populate
<!-- LATENCY_RESULTS_END -->

## Architecture

![image info](./files/diagram.png)

### Components

* `hey`: We use `hey` cli benchmarking tool to cause synthetic load to benchmark the different router `implementations`. We benchmark for '10 seconds` using `200 connections`. We constructed three different request payload configurations: [big](./scripts/bench-hey-big.json), [medium](./scripts/bench-hey-medium.json), [small](./scripts/bench-hey-small.json). Each configuration queries a response of payload size of [112,838 bytes](./source/big.json), [12598 bytes](./source/medium.json), and [362 bytes](./source/small.json) respectively.
* `Implementations`: We use a collection of different federation implementations, and for each of them, we also have different configuration setups located in the [configurations folder](./configurations/). We benchmark each implementation with varying configurations for every data configuration setup (big, medium, small). Most federation router `implementations` make GraphQL requests to the `Mock API` except `Tailcall`, which follows a different approach, enabling regular `Rest API` calls.
* `Mock API`: This component provides data to the `implementations`. It mocks a GraphQL subgraph and a Rest API. This component is written in Rust and serves static data. We do that to eliminate any overheads caused by processing the request in a real GraphQL subgraph.

## Quick Start

To run the benchmarks, you have to install Docker on your computer. We advise using Docker because it eliminates the hassle of managing benchmark dependencies. Follow the instructions provided on the official website: https://docs.docker.com/engine/install/

"`bash
git clone git@github.com:tailcallhq/federation-example.git
cd federation-example
sudo docker build -t tailcallhq/federation-benchmark .
sudo Docker run tailcallhq/federation-benchmark:latest ./benchmark_all.sh
```

## Resources

* [Docker](https://www.docker.com/): Docker is a set of platform-as-a-service products that use OS-level virtualization to deliver software in packages called containers.
* [Hey](https://github.com/rakyll/hey): hey is a tiny program that sends some load to a web application.
* [Rust](https://www.rust-lang.org/): Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory.
* [GraphQL Federation](https://graphql.com/learn/federated-architecture/): GraphQL Federation is an architecture that allows multiple independent GraphQL services to form a unified graph that appears as a single graph to clients. It is a powerful way to scale and manage microservices architecture when using GraphQL.

## Contribute

Your insights are invaluable! Test these benchmarks, share feedback, or contribute by adding more GraphQL frameworks or refining existing ones. Please open an issue or a pull request, and let's build a robust benchmarking resource together!
