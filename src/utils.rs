use substreams_solana::pb::sf::solana::r#type::v1::{MessageAddressTableLookup};

pub const ADDRESS_LOOKUP_TABLE: &str = "AddressLookupTab1e1111111111111111111111111";
pub const ADDRESS_LOOKUP_TABLE_EXTEND_TABLE_INSTRUCTION: [u8; 4] = [2, 0, 0, 0];


pub fn parse_new_addresses(addresses: &[u8]) -> Vec<String> {
    let mut new_addresses = vec![];

    let number_of_addresses = addresses.len() / 32;
    for i in 0..number_of_addresses {
        if i == number_of_addresses {
            break;
        }
        let a = addresses[(i * 32)..(i + 1) * 32].to_vec();
        new_addresses.push(bs58::encode(a).into_string())
    }

    new_addresses
}

pub fn fetch_table_lookup_addresses(address_table_lookups: Vec<MessageAddressTableLookup>) -> Vec<String> {
    return address_table_lookups.into_iter()
        .map(|val| bs58::encode(val.account_key).into_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_new_addresses;

    #[test]
    fn test_parse_new_addresses() {
        struct Case {
            name: String,
            input: Vec<u8>,
            expected_addresses: Vec<String>,
        }

        let case = Case {
            name: "multiple accounts".to_string(),
            input: vec![
                237, 231, 230, 250, 137, 55, 19, 37, 120, 93, 19, 216, 54, 107, 151, 114, 155, 24, 141, 211, 190, 191, 218, 243, 87, 87, 54, 115, 166, 12, 188, 22, 162, 139, 100, 91, 179, 213, 6, 44, 73, 187, 105, 43, 1, 170, 52, 130, 216, 82, 234, 12, 95, 37, 109, 124, 179, 40, 191, 196, 220, 149, 5, 70, 225, 202, 101, 93, 88, 45, 114, 49, 60, 190, 66, 114, 194, 187, 188, 16, 27, 249, 46, 41, 245, 210, 142, 237, 201, 247, 189, 71, 162, 120, 59, 231, 126, 84, 119, 26, 87, 166, 241, 76, 169, 228, 2, 213, 74, 238, 69, 247, 55, 138, 202, 54, 92, 123, 22, 154, 126, 200, 63, 81, 130, 178, 152, 240, 138, 239, 125, 247, 227, 166, 116, 164, 62, 209, 129, 86, 34, 14, 3, 6, 250, 71, 135, 139, 147, 32, 126, 36, 149, 229, 158, 237, 187, 218, 76, 161],
            expected_addresses: vec![
                "H1gikkvnijbQeGLi3tk7RgY4nhiTkqQAmyHX6fBduwbj".to_owned(),
                "BwWKuqJyKVTYyaQJzSouQQ9pbuocNasYH3dK9RDRQEbf".to_owned(),
                "GCPjTU3KDUyHUtHh2Z6d1fvHvQztGn9T1yif7ENEAYgW".to_owned(),
                "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP".to_owned(),
                "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6".to_owned(),
            ],
        };

        assert_eq!(case.expected_addresses, parse_new_addresses(case.input.as_slice()));
    }
}