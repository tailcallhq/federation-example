## Environment

In order to run our examples we have to setup some dependencies that our Demo relies on.
Follow the link for each dependency in order to install it on your computer.

- **Rust:** Install rust by following the guide here https://www.rust-lang.org/tools/install
- **Tailcall:** Install tailcall by following the guide here https://tailcall.run/docs/
- **WRK:** Install wkr by using the installing the pre-build binaries (https://formulae.brew.sh/formula/wrk)

## Setup

We have two different setups, the first one `Demo` is used to show the features of Tailcall,
while the second `Benchmark` is used to show the performance throughput of Tailcall.

### Demo

The `Demo` is used to showcase the Tailcall capabilities and features.
You can start reading to explore the features by reading the `main.graphql` file
and diving deeper by reading each subgraph graphql file in its corresponded folder.
Make sure to visit our [documentation](https://tailcall.run/docs/tailcall-dsl-graphql-custom-directives/) to learn more about the features.

- Run `./run.sh` to start the services.
- Run `tailcall start main.graphql`

### Benchmark

The benchmark is a custom configuration that enables various performance flags on Tailcall.
Keep in mind that those flags can give more performance, but make debugging harder, so its
advised to use them during production setups.

- Run `./run.sh` to start the services.
- Run `TC_LOG_LEVEL=error tailcall start bench.graphql`

Before benchmarking we have to ensure that all services and tailcall are running.
To test each service we can just inspect the terminal where the `./run.sh` command
was executed. For tailcall we check if the terminal has any error, and if not we
can also check by visiting the tailcall [playground](https://tailcall.run/playground/?u=http://127.0.0.1:8030/graphql&utm_source=tailcall-debug&utm_medium=server).

### Sample Query

We can run the following query to ensure that everything is running smoothly.

```gql
query Bench {
  employees {
    details {
      forename
    }
  }
}
```

## Benchmarking

We can benchmark the baseline HTTP API using the following command:

```
wrk -t12 -c200 -d30s http://127.0.0.1:8080/employees
```

On the next step we can benchmark the Tailcall Platform by running:

```
wrk -t12 -c200 -d30s -s bench.lua http://127.0.0.1:8030/graphql
```

## Configurations

In the folder `./configuration` we include the minimal versions of the
benchmarking configurations. We used those configurations to calculate
the performance of Tailcall in different scenarios and features enabled.
