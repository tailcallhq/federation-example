# GraphQL Federation Benchmarks

Explore and compare the performance of the fastest GraphQL federation routers through our comprehensive benchmarks.

- [GraphQL Federation Benchmarks](#graphql-federation-benchmarks)
  - [Benchmark Results](#benchmark-results)
    - [Small Payload - 362 bytes](#small-payload---362-bytes)
    - [Medium Payload - 12,598 bytes](#medium-payload---12598-bytes)
    - [Big Payload - 112,838 bytes](#big-payload---112838-bytes)
    - [RPS Plots](#rps-plots)
    - [Latency 95% Plots](#latency-95-plots)
  - [Architecture](#architecture)
    - [Components](#components)
    - [Specifications](#specifications)
  - [Quick Start](#quick-start)
  - [Resources](#resources)

## Benchmark Results

<!-- PERFORMANCE_RESULTS_START -->
### [Small Payload - 362 bytes](./source/small.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `5,054 RPS` | `0.0175 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `8,404 RPS` | `0.0098 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `4,529 RPS` | `0.0193 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,687 RPS` | `0.0377 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `2,662 RPS` | `0.0264 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `17,861 RPS` | `0.0082 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `7,061 RPS` | `0.0126 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `5,111 RPS` | `0.0243 sec` |
### [Medium Payload - 12,598 bytes](./source/medium.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `4,693 RPS` | `0.0174 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `4,990 RPS` | `0.0143 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `3,637 RPS` | `0.0239 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `1,548 RPS` | `0.0546 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `2,326 RPS` | `0.0303 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `12,048 RPS` | `0.0086 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `5,096 RPS` | `0.018 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `2,909 RPS` | `0.0404 sec` |
### [Big Payload - 112,838 bytes](./source/big.json)
| Server | Status | RPS | Latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | ✅ | `2,097 RPS` | `0.0407 sec` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `1,840 RPS` | `0.0365 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `1,970 RPS` | `0.0343 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `797 RPS` | `0.1086 sec` |
| [Apollo](https://github.com/apollographql/router) | ✅ | `902 RPS` | `0.0967 sec` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | ✅ | `8,151 RPS` | `0.0107 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | ✅ | `2,435 RPS` | `0.032 sec` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | ✅ | `1,133 RPS` | `0.0905 sec` |
<!-- PERFORMANCE_RESULTS_END -->

### RPS Plots

![Requests Per Second: small payload](./files/rps_small.png)

![Requests Per Second: medium payload](./files/rps_medium.png)

![Requests Per Second: big payload](./files/rps_big.png)

### Latency 95% Plots

![Latency 95%: small payload](./files/p95_small.png)

![Latency 95%: medium payload](./files/p95_medium.png)

![Latency 95%: big payload](./files/p95_big.png)

## Architecture

![Architecture Image](./files/architecture.svg)

### Components

1. `hey`: We use `hey` cli benchmarking tool to cause synthetic load to benchmark the different router `implementations`. We benchmark for '10 seconds`using`200 connections`. We constructed three different request payload configurations: [big](./scripts/bench-hey-big.json), [medium](./scripts/bench-hey-medium.json), [small](./scripts/bench-hey-small.json). Each configuration queries a response of payload size of [112,838 bytes](./source/big.json), [12598 bytes](./source/medium.json), and [362 bytes](./source/small.json) respectively.
2. `Implementations`: We use a collection of different federation implementations, and for each of them, we also have different configuration setups located in the [configurations folder](./configurations/). We benchmark each implementation with varying configurations for every data configuration setup (big, medium, small).
3. `Mock`: This component provides data to the `implementations`. It mocks a GraphQL subgraph and an equivalent Rest API. This component is written in Rust and serves static data. We do that to eliminate any overheads caused by processing the request in a real GraphQL subgraph.

### Specifications

Specifications of the machine used for benchmarking:

```
Platform: Linux x64
Size:
2-cores · 8 GB RAM · 75 GB SSD
```

## Quick Start

1. Clone the repository

   ```bash
   git clone git@github.com:tailcallhq/federation-example.git
   cd federation-example
   ```

2. Ensure you have the latest version of [Docker](https://docs.docker.com/engine/install/)

3. Run the following command:
   ```bash
   docker build -t tailcallhq/federation-benchmark .
   docker run tailcallhq/federation-benchmark:latest ./benchmark_all.sh
   ```
4. Wait for benchmarks to complete.

## Resources

- [Docker](https://www.docker.com/): Docker is a set of platform-as-a-service products that use OS-level virtualization to deliver software in packages called containers.
- [Hey](https://github.com/rakyll/hey): hey is a tiny program that sends some load to a web application.
- [Rust](https://www.rust-lang.org/): Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory.
- [GraphQL Federation](https://graphql.com/learn/federated-architecture/): GraphQL Federation is an architecture that allows multiple independent GraphQL services to form a unified graph that appears as a single graph to clients. It is a powerful way to scale and manage microservices architecture when using GraphQL.
