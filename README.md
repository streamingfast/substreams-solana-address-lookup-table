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
```bash
sftoken
make stream
```

## Run the substreams modules with debug outputs
```bash
substreams run substreams.yaml run -e mainnet.sol.streamingfast.io:443 -t +1000 --debug-modules-output=store_address_lookup_tables_stage_2,store_address_lookup_tables
```
