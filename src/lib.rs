mod pb;
mod utils;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams::store::{Appender, DeltaArray, Deltas, StoreGetArray};
use substreams::prelude::{StoreAppend, StoreGet};
use crate::pb::addresslookuptables::types::v1::{AddressLookupTables};

#[substreams::handlers::store]
pub fn store_address_lookup_tables_stage_1(block: Block, store: StoreAppend<String>) {
    for confirmed_trx in block.transactions().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = &confirmed_trx.transaction {
            for compiled_instruction in trx.message.as_ref().unwrap().instructions.iter() {
                let program_id = compiled_instruction.program_id_index as usize;
                let accounts = &trx.message.as_ref().unwrap().account_keys;
                let address_lookup_table_account = bs58::encode(&accounts[program_id]).into_string();

                if address_lookup_table_account != utils::ADDRESS_LOOKUP_TABLE {
                    continue;
                }

                let instruction = <[u8; 4]>::try_from(&compiled_instruction.data[0..4]).unwrap();
                if instruction != utils::ADDRESS_LOOKUP_TABLE_EXTEND_TABLE_INSTRUCTION {
                    continue;
                }

                let new_addresses = utils::parse_new_addresses(&compiled_instruction.data[12..]);
                let table_address_idx = compiled_instruction.accounts[0] as usize;

                if table_address_idx >= accounts.len() {
                    let table_lookup_addresses: Vec<String> = utils::fetch_table_lookup_addresses(&trx);
                    let idx = table_address_idx - accounts.len();

                    store.append_all(
                        0,
                        format!("lookup:{idx}:{}", table_lookup_addresses.join(":")),
                        new_addresses
                    );
                    continue;
                }

                let table_address = bs58::encode(&accounts[table_address_idx]).into_string();

                store.append_all(
                    0,
                    format!("resolved:{table_address}"),
                    new_addresses
                )
            }
        }
    }
}

#[substreams::handlers::store]
pub fn store_address_lookup_tables_stage_2(store_stage_1: StoreGetArray<String>, deltas_stage_1: Deltas<DeltaArray<String>>, store: StoreAppend<String>) {
    deltas_stage_1.deltas.into_iter().for_each(|delta| {
        let parts: Vec<&str> = delta.key.split(":").collect();
        let address = delta.new_value.last().unwrap();
        let table_address;
        let mut resolved_addresses: Vec<String> = vec![];

        match parts[0] {
            "lookup" => {
                let table_idx = parts[1].parse::<usize>().unwrap();
                let table_addresses = parts[2..].to_vec();

                table_addresses.into_iter().for_each(|address| {
                    if let Some(mut vals) = store_stage_1.get_last(format!("resolved:{address}")) {
                        resolved_addresses.append(&mut vals);
                    }
                });

                table_address = resolved_addresses.get(table_idx).unwrap().to_owned();
            },
            "resolved" => {
                table_address = parts[1].to_string();
            },
            _ => panic!("unresolved key")
        }

        store.append(
            0,
            format!("table:{}", table_address),
            address.to_owned()
        );
    })
}

#[substreams::handlers::map]
pub fn run(deltas: Deltas<DeltaArray<String>>) -> Result<AddressLookupTables, Error> {
    let _ = deltas;
    // no ops
    Ok(AddressLookupTables::default())
}