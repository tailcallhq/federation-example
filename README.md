# Federation Benchmarks

Explore and compare the performance of the fastest GraphQL federation routers through our comprehensive benchmarks.

- [Benchmark Results](#benchmark-results)
- [Introduction](#introduction)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Resources](#resources)
- [Contribute](#contribute)

## Benchmark Results

<!-- PERFORMANCE_RESULTS_START -->
| Server | [112,838 bytes](./source/big.json) status / rps / latency | [12,598 bytes](./source/medium.json) status / rps / latency | [362 bytes](./source/small.json) status / rps / latency |
| ---: | ---: | ---: | ---: |
| [Nginx](https://nginx.org/en/) | `❌ / 5,042 RPS / 0.0091 sec` | `❌ / 12,155 RPS / 0.0223 sec` | `❌ / 10,140 RPS / 0.0047 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | `❌ / 3,777 RPS / 0.009 sec` | `❌ / 9,841 RPS / 0.0545 sec` | `❌ / 3,645 RPS / 0.0131 sec` |
| [Apollo](https://github.com/apollographql/router) | `❌ / 2,066 RPS / 0.0167 sec` | `❌ / 5,119 RPS / 0.0045 sec` | `❌ / 31,499 RPS / 0.0021 sec` |
| [Grafbase](https://github.com/grafbase/grafbase) | `❌ / 4,586 RPS / 0.0076 sec` | `❌ / 16,269 RPS / 0.0405 sec` | `❌ / 6,681 RPS / 0.0109 sec` |
<!-- PERFORMANCE_RESULTS_END -->

## Introduction

This document presents a comparative analysis of several renowned GraphQL Federation routers. Dive deep into the performance metrics and get insights regarding their throughput and latency.

> **NOTE:** This is a work-in-progress suite of benchmarks, and we would appreciate help from the community to add more routers or tune the existing ones for better performance.

## Architecture

![image info](./files/diagram.png)

### Components

* `hey`: We use `hey` cli benchmarking tool to cause synthetic load to benchmark the different router `implementations`. We benchmark for '10 seconds` using `200 connections`. We constructed three different request payload configurations: [big](./scripts/bench-hey-big.json), [medium](./scripts/bench-hey-medium.json), [small](./scripts/bench-hey-small.json). Each configuration queries a response of payload size of [112,838 bytes](./source/big.json), [12598 bytes](./source/medium.json), and [362 bytes](./source/small.json) respectively.
* `Implementations`: We use a collection of different federation implementations, and for each of them, we also have different configuration setups located in the [configurations folder](./configurations/). We benchmark each implementation with varying configurations for every data configuration setup (big, medium, small). Most federation router `implementations` make GraphQL requests to the `Mock API` except `Tailcall`, which follows a different approach, enabling regular `Rest API` calls.
* `Mock API`: This component provides data to the `implementations`. It mocks a GraphQL subgraph and a Rest API. This component is written in Rust and serves static data. We do that to eliminate any overheads caused by processing the request in a real GraphQL subgraph.

## Quick Start

To run the benchmarks, you have to install Docker on your computer. We advise using Docker because it eliminates the hassle of managing benchmark dependencies. Follow the instructions provided on the official website: https://docs.docker.com/engine/install/

```bash
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
