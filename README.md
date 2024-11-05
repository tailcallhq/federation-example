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
| [Nginx](https://nginx.org/en/) | `5,292 RPS` | `11,557 RPS` | `12,336 RPS` |
| **Base** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | `3,235 RPS` | `10,024 RPS` | `17,697 RPS` |
| [Grafbase](https://github.com/grafbase/grafbase) | `59,579 RPS` | `59,854 RPS` | `59,831 RPS` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | `2,136 RPS` | `3,603 RPS` | `6,957 RPS` |
| [Apollo](https://github.com/apollographql/router) | `60,540 RPS` | `58,970 RPS` | `60,082 RPS` |
| **Cached** | | | |
| [Tailcall](https://github.com/tailcallhq/tailcall) | `17,690 RPS` | `31,112 RPS` | `40,854 RPS` |
| [Grafbase](https://github.com/grafbase/grafbase) | `60,251 RPS` | `60,175 RPS` | `59,725 RPS` |
| [Wundegraph](https://github.com/wundergraph/cosmo) | `2,933 RPS` | `6,264 RPS` | `11,139 RPS` |
<!-- PERFORMANCE_RESULTS_END -->

## Architecture

TODO: add information

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

## Setup

In order to run the benchmarks you have to install docker in your computer. We advise using docker because it takes the hassle of managing the benchmark dependencies away. Follow the instructions provided in the official website: https://docs.docker.com/engine/install/

## Resources

* [Docker](https://www.docker.com/): Docker is a set of platform as a service products that use OS-level virtualization to deliver software in packages called containers.
* [Hey](https://github.com/rakyll/hey): hey is a tiny program that sends some load to a web application.
* [Rust](https://www.rust-lang.org/): Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory.
* [GraphQL Federation](https://graphql.com/learn/federated-architecture/): GraphQL Federation is an architecture that allows multiple independent GraphQL services to form a unified graph that appears as a single graph to clients. It is a powerful way to scale and manage microservices architecture when using GraphQL.

## Contribute

Your insights are invaluable! Test these benchmarks, share feedback, or contribute by adding more GraphQL frameworks or refining existing ones. Open an issue or a pull request, and let's build a robust benchmarking resource together!
