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
| [Nginx](https://nginx.org/en/) | ✅ | `5,034 RPS` | `0.0193 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `7,084 RPS` | `0.0099 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `5,018 RPS` | `0.0138 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,858 RPS` | `0.0323 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `1,856 RPS` | `0.0449 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `17,214 RPS` | `0.01 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `7,456 RPS` | `0.0108 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `4,869 RPS` | `0.0267 sec` |
### [Medium Payload - 12,598 bytes](./source/medium.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `4,675 RPS` | `0.0193 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `4,563 RPS` | `0.0185 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `4,140 RPS` | `0.0163 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `1,511 RPS` | `0.0675 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `2,029 RPS` | `0.042 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `13,629 RPS` | `0.0068 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `5,437 RPS` | `0.015 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,797 RPS` | `0.0439 sec` |
### [Big Payload - 112,838 bytes](./source/big.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `2,049 RPS` | `0.0403 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `1,579 RPS` | `0.0482 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `1,828 RPS` | `0.048 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `886 RPS` | `0.106 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `1,199 RPS` | `0.0591 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `7,465 RPS` | `0.0117 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `2,371 RPS` | `0.0331 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `1,368 RPS` | `0.0876 sec` |
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
