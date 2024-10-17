## Environment

- **Rust:** Install rust by following the guide here https://www.rust-lang.org/tools/install
- **Tailcall:** Install tailcall by following the guide here https://tailcall.run/docs/
- **K6:** Install k6 benchmarking tool by following the guide here https://grafana.com/docs/k6/latest/set-up/install-k6/

## Setup

### Demo

- Run `./run.sh` to start the services.
- Run `TC_LOG_LEVEL=error tailcall start main.graphql`

### Benchmark

- Run `./run.sh` to start the services.
- Run `TC_LOG_LEVEL=error tailcall start bench.graphql`

Before benchmarking we have to ensure that all services and tailcall are running. To test each service we can just inspect the terminal where the `./run.sh` command was executed. For tailcall we check if the terminal has any error, and if not we can also check by visiting the tailcall [playground](https://tailcall.run/playground/?u=http://127.0.0.1:8030/graphql&utm_source=tailcall-debug&utm_medium=server).

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
k6 run baseline.js
```

On the next step we can benchmark the Tailcall Platform by running:

```
k6 run bench.js
```

## Configurations

In the folder `./configuration` we include the minimal versions of the
benchmarking configurations. We used those configurations to calculate
the performance of Tailcall in different scenarios.
