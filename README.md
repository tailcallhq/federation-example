# Federation Benchmarks

Explore and compare the performance of the fastest GraphQL Federation routers through our comprehensive benchmarks.

- [Introduction](#introduction)
- [Quick Start](#quick-start)
- [Benchmark Results](#benchmark-results)
- [Architecture](#architecture)
- [Setup](#setup)
- [Resources](#resources)
- [Contribute](#contribute)

## Introduction

This document presents a comparative analysis of several renowned GraphQL Federation routers. Dive deep into the performance metrics, and get insights into their throughput.

> **NOTE:** This is a work in progress suite of benchmarks, and we would appreciate help from the community to add more routers or tune the existing ones for better performance.

## Quick Start

```bash
git clone git@github.com:tailcallhq/federation-example.git
cd federation-example
sudo docker build -t tailcallhq/federation-benchmark
sudo docker run tailcallhq/federation-benchmark:latest ./benchmark_all.sh
```

## Benchmark Results

<!-- PERFORMANCE_RESULTS_START -->
| Server | 112,838 bytes | 12598 bytes | 362 bytes |
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

## Architecture

![image info](./files/diagram.png)

### Components

* `hey`: We use `hey` cli benchmarking tool to cause synthetic load in order to benchmark the `router` component. We benchmark for `10 seconds` using `200 threads` using `POST` method with graphql payload. The settings for the benchmarks are in the `bench-hey-**.json` files.
* `router`: This is the component we are targeting to benchmark. We use a collection of different federation implementation with different settings to have a comprehensive comparison.
* `subgraph`: This component is used to provide data to the `router` component. It mimics a GraphQL subgraph, but in reality it serves a static response. We do that so we can eliminate any overheads that are caused by processing the request in a real GraphQL subgraph.

### Query

For both `big` and `medium` we use the following query. It provides a deeply nested structure, and with different response payloads it can be a challenge on some federation implementations to parse it.

```gql
query BigQuery($delay: Int!, $bigObjects: Int!, $deeplyNestedObjects: Int!, $nestedObjects: Int!) { bigResponse( artificialDelay: $delay bigObjects: $bigObjects deeplyNestedObjects: $deeplyNestedObjects nestedObjects: $nestedObjects ) { a: nestedObjects { ...DeeplyNestedFields } b: nestedObjects { ...DeeplyNestedFields } c: nestedObjects { ...DeeplyNestedFields } d: nestedObjects { ...DeeplyNestedFields } e: nestedObjects { ...DeeplyNestedFields } f: nestedObjects { ...DeeplyNestedFields } } } fragment DeeplyNestedFields on NestedObject { deeplyNestedObjects { aFieldOnDeeplyNestedObject bFieldOnDeeplyNestedObject cFieldOnDeeplyNestedObject dFieldOnDeeplyNestedObject eFieldOnDeeplyNestedObject fFieldOnDeeplyNestedObject gFieldOnDeeplyNestedObject hFieldOnDeeplyNestedObject iFieldOnDeeplyNestedObject jFieldOnDeeplyNestedObject kFieldOnDeeplyNestedObject lFieldOnDeeplyNestedObject mFieldOnDeeplyNestedObject nFieldOnDeeplyNestedObject oFieldOnDeeplyNestedObject pFieldOnDeeplyNestedObject qFieldOnDeeplyNestedObject rFieldOnDeeplyNestedObject sFieldOnDeeplyNestedObject tFieldOnDeeplyNestedObject uFieldOnDeeplyNestedObject vFieldOnDeeplyNestedObject wFieldOnDeeplyNestedObject xFieldOnDeeplyNestedObject yFieldOnDeeplyNestedObject zFieldOnDeeplyNestedObject } }
```

For the `small` payload setup we use the following query, that challenges the latency of the implementations.

```gql
query {
  employees {
    details {
      forename
    }
  }
}
```

## Setup

In order to run the benchmarks you have to install docker in your computer. We advise using docker because it takes the hassle of managing the benchmark dependencies away. Follow the instructions provided in the official website: https://docs.docker.com/engine/install/

## Resources

* [Docker](https://www.docker.com/): Docker is a set of platform as a service products that use OS-level virtualization to deliver software in packages called containers.
* [Hey](https://github.com/rakyll/hey): hey is a tiny program that sends some load to a web application.
* [Rust](https://www.rust-lang.org/): Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory.
* [GraphQL Federation](https://graphql.com/learn/federated-architecture/): GraphQL Federation is an architecture that allows multiple independent GraphQL services to form a unified graph that appears as a single graph to clients. It is a powerful way to scale and manage microservices architecture when using GraphQL.

## Contribute

Your insights are invaluable! Test these benchmarks, share feedback, or contribute by adding more GraphQL frameworks or refining existing ones. Open an issue or a pull request, and let's build a robust benchmarking resource together!
