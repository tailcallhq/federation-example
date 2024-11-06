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
| [Nginx](https://nginx.org/en/) | ✅ | `12,234 RPS` | `0.011 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `17,442 RPS` | `0.0044 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `10,527 RPS` | `0.0069 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `6,879 RPS` | `0.0132 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `4,599 RPS` | `0.016 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `41,395 RPS` | `0.0022 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `16,871 RPS` | `0.0054 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `10,571 RPS` | `0.0112 sec` |
### [Medium Payload - 12,598 bytes](./source/medium.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `11,571 RPS` | `0.0107 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `10,378 RPS` | `0.007 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `7,965 RPS` | `0.0093 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `3,832 RPS` | `0.0212 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `3,857 RPS` | `0.0193 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `30,531 RPS` | `0.0029 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `12,257 RPS` | `0.0069 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `6,473 RPS` | `0.0172 sec` |
### [Big Payload - 112,838 bytes](./source/big.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `5,065 RPS` | `0.0186 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `3,260 RPS` | `0.0224 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `3,727 RPS` | `0.0201 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,160 RPS` | `0.0557 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `2,011 RPS` | `0.0384 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `17,920 RPS` | `0.0044 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `4,919 RPS` | `0.0171 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,905 RPS` | `0.0408 sec` |
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
