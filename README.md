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

|                                             Server | Status |          RPS |      Latency |
| -------------------------------------------------: | -----: | -----------: | -----------: |
|                     [Nginx](https://nginx.org/en/) |     ✅ | `12,077 RPS` | `0.0082 sec` |
|                                           **Base** |        |              |              |
| [Tailcall](https://github.com/tailcallhq/tailcall) |     ✅ | `16,685 RPS` | `0.0048 sec` |
|   [Grafbase](https://github.com/grafbase/grafbase) |     ✅ | `10,412 RPS` | `0.0069 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) |     ✅ |  `7,595 RPS` | `0.0114 sec` |
|  [Apollo](https://github.com/apollographql/router) |     ✅ |  `4,700 RPS` | `0.0157 sec` |
|                                         **Cached** |        |              |              |
| [Tailcall](https://github.com/tailcallhq/tailcall) |     ✅ | `41,546 RPS` | `0.0022 sec` |
|   [Grafbase](https://github.com/grafbase/grafbase) |     ✅ | `18,144 RPS` | `0.0051 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) |     ✅ | `11,048 RPS` | `0.0106 sec` |

### [Medium Payload - 12,598 bytes](./source/medium.json)

|                                             Server | Status |          RPS |      Latency |
| -------------------------------------------------: | -----: | -----------: | -----------: |
|                     [Nginx](https://nginx.org/en/) |     ✅ |  `9,797 RPS` | `0.0084 sec` |
|                                           **Base** |        |              |              |
| [Tailcall](https://github.com/tailcallhq/tailcall) |     ✅ | `10,082 RPS` | `0.0073 sec` |
|   [Grafbase](https://github.com/grafbase/grafbase) |     ✅ |  `8,062 RPS` | `0.0092 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) |     ✅ |  `3,674 RPS` | `0.0228 sec` |
|  [Apollo](https://github.com/apollographql/router) |     ✅ |  `4,198 RPS` | `0.0172 sec` |
|                                         **Cached** |        |              |              |
| [Tailcall](https://github.com/tailcallhq/tailcall) |     ✅ | `30,504 RPS` | `0.0028 sec` |
|   [Grafbase](https://github.com/grafbase/grafbase) |     ✅ | `11,677 RPS` | `0.0075 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) |     ✅ |  `6,332 RPS` | `0.0181 sec` |

### [Big Payload - 112,838 bytes](./source/big.json)

|                                             Server | Status |          RPS |      Latency |
| -------------------------------------------------: | -----: | -----------: | -----------: |
|                     [Nginx](https://nginx.org/en/) |     ✅ |  `5,065 RPS` | `0.0235 sec` |
|                                           **Base** |        |              |              |
| [Tailcall](https://github.com/tailcallhq/tailcall) |     ✅ |  `3,204 RPS` | `0.0227 sec` |
|   [Grafbase](https://github.com/grafbase/grafbase) |     ✅ |  `3,780 RPS` | `0.0198 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) |     ✅ |  `2,144 RPS` | `0.0551 sec` |
|  [Apollo](https://github.com/apollographql/router) |     ✅ |  `2,017 RPS` | `0.0381 sec` |
|                                         **Cached** |        |              |              |
| [Tailcall](https://github.com/tailcallhq/tailcall) |     ✅ | `17,820 RPS` | `0.0044 sec` |
|   [Grafbase](https://github.com/grafbase/grafbase) |     ✅ |  `4,592 RPS` |  `0.019 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) |     ✅ |  `2,903 RPS` |  `0.041 sec` |

<!-- PERFORMANCE_RESULTS_END -->

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
