## Environment

In order to run our examples we have to setup some dependencies that our Demo relies on.
Follow the link for each dependency in order to install it on your computer.

- **Rust:** Install rust by following the guide here https://www.rust-lang.org/tools/install
- **Tailcall:** Install tailcall by following the guide here https://tailcall.run/docs/
- **hey:** Install hey by following the instructions on the page here https://github.com/rakyll/hey

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
query BigQuery($delay: Int!, $bigObjects: Int!, $deeplyNestedObjects: Int!, $nestedObjects: Int!) { bigResponse( artificialDelay: $delay bigObjects: $bigObjects deeplyNestedObjects: $deeplyNestedObjects nestedObjects: $nestedObjects ) { a: nestedObjects { ...DeeplyNestedFields } b: nestedObjects { ...DeeplyNestedFields } c: nestedObjects { ...DeeplyNestedFields } d: nestedObjects { ...DeeplyNestedFields } e: nestedObjects { ...DeeplyNestedFields } f: nestedObjects { ...DeeplyNestedFields } } } fragment DeeplyNestedFields on NestedObject { deeplyNestedObjects { aFieldOnDeeplyNestedObject bFieldOnDeeplyNestedObject cFieldOnDeeplyNestedObject dFieldOnDeeplyNestedObject eFieldOnDeeplyNestedObject fFieldOnDeeplyNestedObject gFieldOnDeeplyNestedObject hFieldOnDeeplyNestedObject iFieldOnDeeplyNestedObject jFieldOnDeeplyNestedObject kFieldOnDeeplyNestedObject lFieldOnDeeplyNestedObject mFieldOnDeeplyNestedObject nFieldOnDeeplyNestedObject oFieldOnDeeplyNestedObject pFieldOnDeeplyNestedObject qFieldOnDeeplyNestedObject rFieldOnDeeplyNestedObject sFieldOnDeeplyNestedObject tFieldOnDeeplyNestedObject uFieldOnDeeplyNestedObject vFieldOnDeeplyNestedObject wFieldOnDeeplyNestedObject xFieldOnDeeplyNestedObject yFieldOnDeeplyNestedObject zFieldOnDeeplyNestedObject } }
```

## Benchmarking

On the next step we can benchmark the Tailcall Platform by running:

```
hey -n 200 -z 10s -m POST -H 'Accept: application/json' -H 'Content-Type: application/json' -D bench.json http://127.0.0.1:8030/graphql
```

## Configurations

In the folder `./configuration` we can read reference upstream configurations with different performance gains.
