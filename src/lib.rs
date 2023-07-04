mod pb;

use substreams::errors::Error;
use substreams_solana::pb::sol::v1::Block;
use substreams::store::{Appender, DeltaArray, Deltas};
use substreams::prelude::StoreAppend;
use crate::pb::addresslookuptables::types::v1::{AddressLookupTable, AddressLookupTables};

const ADDRESS_LOOKUP_TABLE: &str = "AddressLookupTab1e1111111111111111111111111";
const ADDRESS_LOOKUP_TABLE_EXTEND_TABLE_INSTRUCTION: [u8; 4] = [2, 0, 0, 0];

#[substreams::handlers::map]
pub fn map_address_lookup_tables(block: Block) -> Result<AddressLookupTables, Error> {
    let mut address_lookup_tables = vec![];

    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            for compiled_instruction in trx.message.as_ref().unwrap().instructions.iter() {
                let program_id = compiled_instruction.program_id_index as usize;
                let address_lookup_table_account = bs58::encode(&trx.message.as_ref().unwrap().account_keys[program_id]).into_string();

                if address_lookup_table_account != ADDRESS_LOOKUP_TABLE {
                    continue;
                }

                let instruction = <[u8; 4]>::try_from(&compiled_instruction.data[0..4]).unwrap();
                if instruction != ADDRESS_LOOKUP_TABLE_EXTEND_TABLE_INSTRUCTION {
                    continue;
                }

                let table_address = bs58::encode(&trx.message.as_ref().unwrap().account_keys[compiled_instruction.accounts[0] as usize]).into_string();
                let addresses = &compiled_instruction.data[12..];
                let mut new_addresses = vec![];

                let number_of_addresses = addresses.len() / 32;
                for i in 0..number_of_addresses {
                    if i == number_of_addresses {
                        break;
                    }
                    let a = addresses[(i * 32)..(i + 1) * 32].to_vec();
                    new_addresses.push(bs58::encode(a).into_string())
                }

                address_lookup_tables.push(AddressLookupTable {
                    table_address,
                    addresses: new_addresses,
                });
            }
        }
    }

    Ok(AddressLookupTables { address_lookup_tables })
}

#[substreams::handlers::store]
pub fn store_address_lookup_tables(address_lookup_tables: AddressLookupTables, store: StoreAppend<String>) {
    for address_lookup_table in address_lookup_tables.address_lookup_tables {
        store.append_all(
            1,
            format!("table:{}", address_lookup_table.table_address),
            address_lookup_table.addresses
        );
    }
}

#[substreams::handlers::map]
pub fn run(deltas: Deltas<DeltaArray<String>>) -> Result<AddressLookupTables, Error> {
    let _ = deltas;
    // no ops
    Ok(AddressLookupTables::default())
}