# substreams-solana-address-lookup-table
Substreams containing a store with all the address look up table addresses for solana

## Build proto
```bash
make protogen
```

## Build substreams
```bash
make build
```

## Run the substreams
```
sftoken
make stream
```
> You can add the debug flag: `---debug-modules-output=store_address_lookup_tables_stage_2,store_address_lookup_tables_stage_1` (to the substreams command) to see the output and the logs of the stores
