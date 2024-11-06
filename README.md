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
| [Nginx](https://nginx.org/en/) | ✅ | `4,921 RPS` | `0.0202 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `7,800 RPS` | `0.0125 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `5,061 RPS` | `0.0145 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `3,250 RPS` | `0.026 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `2,629 RPS` | `0.0264 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `16,585 RPS` | `0.0097 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `7,564 RPS` | `0.0107 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `5,634 RPS` | `0.0214 sec` |
### [Medium Payload - 12,598 bytes](./source/medium.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `4,665 RPS` | `0.0236 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `4,847 RPS` | `0.0147 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `3,664 RPS` | `0.0237 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `1,835 RPS` | `0.0447 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `1,764 RPS` | `0.0477 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `11,794 RPS` | `0.0089 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `5,709 RPS` | `0.0137 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `3,052 RPS` | `0.0395 sec` |
### [Big Payload - 112,838 bytes](./source/big.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `2,009 RPS` | `0.0466 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `1,642 RPS` | `0.0412 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `1,992 RPS` | `0.0341 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `1,061 RPS` | `0.0862 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `987 RPS` | `0.09 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `7,410 RPS` | `0.0122 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `2,162 RPS` | `0.0408 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `1,364 RPS` | `0.0892 sec` |
<!-- PERFORMANCE_RESULTS_END -->

### RPS Plots

![Requests Per Second: bar plot, default configuration](./files/rps_default.png)
![Requests Per Second: bar plot, cached configuration](./files/rps_cached.png)

### Latency 95%

![Latency 95%: bar plot, default configuration](./files/p95_default.png)
![Latency 95%: bar plot, cached configuration](./files/p95_cached.png)

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
