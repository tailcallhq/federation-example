## Environment

In order to run our examples we have to setup some dependencies that our Demo relies on.
Follow the link for each dependency in order to install it on your computer.

### Demo

- **Rust:** https://www.rust-lang.org/tools/install/
- **Tailcall:** https://tailcall.run/docs/

### Benchmark

- **hey:** https://github.com/rakyll/hey/
- **Docker:**: https://docs.docker.com/engine/install/

## Setup

We have two different setups, the first one `Demo` is used to show the features of Tailcall,
while the second `Benchmark` is used to show the performance throughput of Tailcall.

### Demo

The `Demo` is used to showcase the Tailcall capabilities and features.
You can start reading to explore the features by reading the `main.graphql` file
and diving deeper by reading each subgraph graphql file in its corresponded folder.
Make sure to visit our [documentation](https://tailcall.run/docs/tailcall-dsl-graphql-custom-directives/) to learn more about the features.

#### Steps:

- Run `./run.sh` to start the services
- Run `tailcall start main.graphql`

#### Sample Query

You can now navigate to Tailcall's [playground](https://tailcall.run/playground/?u=http://127.0.0.1:8030/graphql&utm_source=tailcall-debug&utm_medium=server) and explore the capabilities.
We can run the following query to ensure that everything is running smoothly.

```gql
query {
  employees {
    details {
      forename
    }
  }
}
```

### Benchmark

The benchmark is a custom configuration that enables various performance flags on Tailcall.
Keep in mind that those flags can give more performance, but make debugging harder, so its
advised to use them during production setups.


#### Steps:

- Run `cd nginx && docker build -t tc-benchmark . && docker run -d -p 4006:4006 -p 8090:8090 -p 4001:4001 -p 8091:8091  --name tc-benchmark tc-benchmark`

  To run the static upstream server. This server provides data to tailcall.

- Run `TC_LOG_LEVEL=error tailcall start ./configurations/1-basic.graphql`

  To run the Tailcall platform. You can now navigate to Tailcall's [playground](https://tailcall.run/playground/?u=http://127.0.0.1:8030/graphql&utm_source=tailcall-debug&utm_medium=server) and run the reference query to ensure that everything is running smoothly.


- Run `hey -n 200 -z 10s -m GET -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:8090/big-json`

  Benchmark the baseline NGINX->NGINX implementation and measure statistics.

- Run `hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D bench-hey.json http://127.0.0.1:8030/graphql`

  Benchmark the Tailcall platform implementation and measure statistics.

In the folder `./configuration` we can read reference upstream configurations.

#### Sample Query

You can now navigate to Tailcall's [playground](https://tailcall.run/playground/?u=http://127.0.0.1:8030/graphql&utm_source=tailcall-debug&utm_medium=server) and run the following query to ensure that everything is running smoothly.

```gql
query BigQuery($delay: Int!, $bigObjects: Int!, $deeplyNestedObjects: Int!, $nestedObjects: Int!) { bigResponse( artificialDelay: $delay bigObjects: $bigObjects deeplyNestedObjects: $deeplyNestedObjects nestedObjects: $nestedObjects ) { a: nestedObjects { ...DeeplyNestedFields } b: nestedObjects { ...DeeplyNestedFields } c: nestedObjects { ...DeeplyNestedFields } d: nestedObjects { ...DeeplyNestedFields } e: nestedObjects { ...DeeplyNestedFields } f: nestedObjects { ...DeeplyNestedFields } } } fragment DeeplyNestedFields on NestedObject { deeplyNestedObjects { aFieldOnDeeplyNestedObject bFieldOnDeeplyNestedObject cFieldOnDeeplyNestedObject dFieldOnDeeplyNestedObject eFieldOnDeeplyNestedObject fFieldOnDeeplyNestedObject gFieldOnDeeplyNestedObject hFieldOnDeeplyNestedObject iFieldOnDeeplyNestedObject jFieldOnDeeplyNestedObject kFieldOnDeeplyNestedObject lFieldOnDeeplyNestedObject mFieldOnDeeplyNestedObject nFieldOnDeeplyNestedObject oFieldOnDeeplyNestedObject pFieldOnDeeplyNestedObject qFieldOnDeeplyNestedObject rFieldOnDeeplyNestedObject sFieldOnDeeplyNestedObject tFieldOnDeeplyNestedObject uFieldOnDeeplyNestedObject vFieldOnDeeplyNestedObject wFieldOnDeeplyNestedObject xFieldOnDeeplyNestedObject yFieldOnDeeplyNestedObject zFieldOnDeeplyNestedObject } }
```

## HEY Benchmarking

**Big**

We can benchmark the baseline using the following command:

```
hey -n 200 -z 10s -m GET -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:8090/big-json
```

On the next step we can benchmark the Tailcall Platform by running:

```
hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D bench-hey-big.json http://127.0.0.1:8030/graphql
```

**Medium**

**Before conducting this test replace `big` with `medium` in `nginx.conf` and rebuild.**

We can benchmark the baseline using the following command:

```
hey -n 200 -z 10s -m GET -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:8090/medium-json
```

On the next step we can benchmark the Tailcall Platform by running:

```
hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D bench-hey-medium.json http://127.0.0.1:8030/graphql
```

**Small**

We can benchmark the baseline using the following command:

```
hey -n 200 -z 10s -m GET -H 'Accept: application/json' -H 'Content-Type: application/json' http://127.0.0.1:8091/employees
```

On the next step we can benchmark the Tailcall Platform by running:

```
hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D bench-hey-small.json http://127.0.0.1:8030/graphql
```

## WRK Benchmarking

We can benchmark the baseline using the following command:

```
wrk -t12 -c200 -d10s http://127.0.0.1:8090/big-json
```

On the next step we can benchmark the Tailcall Platform by running:

```
wrk -t12 -c200 -d10s -s bench-wrk.lua http://127.0.0.1:8030/graphql
```
