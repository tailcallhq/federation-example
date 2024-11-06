# GraphQL Federation Benchmarks

Explore and compare the performance of the fastest GraphQL federation routers through our comprehensive benchmarks.

- [GraphQL Federation Benchmarks](#graphql-federation-benchmarks)
  - [Benchmark Results](#benchmark-results)
    - [Small Payload - 362 bytes](#small-payload---362-bytes)
    - [Medium Payload - 12,598 bytes](#medium-payload---12598-bytes)
    - [Big Payload - 112,838 bytes](#big-payload---112838-bytes)
  - [Architecture](#architecture)
    - [Components](#components)
  - [Quick Start](#quick-start)
  - [Resources](#resources)

## Benchmark Results

<!-- PERFORMANCE_RESULTS_START -->
### [Small Payload - 362 bytes](./source/small.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `12,075 RPS` | `0.0091 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `17,083 RPS` | `0.0047 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `9,978 RPS` | `0.0075 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `6,927 RPS` | `0.0132 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `4,553 RPS` | `0.0161 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `40,558 RPS` | `0.0022 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `16,967 RPS` | `0.0054 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `10,763 RPS` | `0.011 sec` |
### [Medium Payload - 12,598 bytes](./source/medium.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `11,426 RPS` | `0.0108 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `10,005 RPS` | `0.0074 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `8,394 RPS` | `0.0086 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `3,702 RPS` | `0.0224 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `4,146 RPS` | `0.0181 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `30,781 RPS` | `0.0028 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `12,354 RPS` | `0.007 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `6,339 RPS` | `0.018 sec` |
### [Big Payload - 112,838 bytes](./source/big.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `5,299 RPS` | `0.0163 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `3,261 RPS` | `0.0225 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `3,810 RPS` | `0.0198 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,174 RPS` | `0.0546 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `2,282 RPS` | `0.0326 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `17,631 RPS` | `0.0045 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `4,609 RPS` | `0.0188 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,933 RPS` | `0.0394 sec` |
<!-- PERFORMANCE_RESULTS_END -->

Specifications of the machine used for benchmarking:

```
Platform: Linux x64
Size:
2-cores · 8 GB RAM · 75 GB SSD
```

## Architecture

![Architecture Image](./files/architecture.svg)

### Components

1. `hey`: We use `hey` cli benchmarking tool to cause synthetic load to benchmark the different router `implementations`. We benchmark for '10 seconds`using`200 connections`. We constructed three different request payload configurations: [big](./scripts/bench-hey-big.json), [medium](./scripts/bench-hey-medium.json), [small](./scripts/bench-hey-small.json). Each configuration queries a response of payload size of [112,838 bytes](./source/big.json), [12598 bytes](./source/medium.json), and [362 bytes](./source/small.json) respectively.
2. `Implementations`: We use a collection of different federation implementations, and for each of them, we also have different configuration setups located in the [configurations folder](./configurations/). We benchmark each implementation with varying configurations for every data configuration setup (big, medium, small).
3. `Mock`: This component provides data to the `implementations`. It mocks a GraphQL subgraph and an equivalent Rest API. This component is written in Rust and serves static data. We do that to eliminate any overheads caused by processing the request in a real GraphQL subgraph.

## Quick Start

To run the benchmarks, you have to install Docker on your computer. We advise using Docker because it eliminates the hassle of managing benchmark dependencies. Follow the instructions provided on the official website: https://docs.docker.com/engine/install/

```bash
git clone git@github.com:tailcallhq/federation-example.git
cd federation-example
sudo docker build -t tailcallhq/federation-benchmark .
sudo Docker run tailcallhq/federation-benchmark:latest ./benchmark_all.sh
```

## Resources

- [Docker](https://www.docker.com/): Docker is a set of platform-as-a-service products that use OS-level virtualization to deliver software in packages called containers.
- [Hey](https://github.com/rakyll/hey): hey is a tiny program that sends some load to a web application.
- [Rust](https://www.rust-lang.org/): Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory.
- [GraphQL Federation](https://graphql.com/learn/federated-architecture/): GraphQL Federation is an architecture that allows multiple independent GraphQL services to form a unified graph that appears as a single graph to clients. It is a powerful way to scale and manage microservices architecture when using GraphQL.
