use substreams_solana::pb::sol::v1::Block;
use substreams::store::Appender;
use substreams::prelude::StoreAppend;

#[substreams::handlers::store]
pub fn store_address_lookup_tables(block: Block, output: StoreAppend<String>) {
    //todo
}