## Environment

In order to run our examples we have to setup some dependencies that our Demo relies on.
Follow the link for each dependency in order to install it on your computer.

- **Rust:** Install rust by following the guide here https://www.rust-lang.org/tools/install
- **Tailcall:** Install tailcall by following the guide here https://tailcall.run/docs/

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
