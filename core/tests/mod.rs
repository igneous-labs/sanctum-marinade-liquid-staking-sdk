#[cfg(test)]
mod tests {
    use const_crypto::bs58;
    use data_encoding::BASE64;
    use sanctum_marinade_liquid_staking_core::{self as marinade_staking_sdk};

    const MARINADE_STATE_DATA: &[u8] = include_bytes!("../../test-fixtures/marinade-state.json");

    #[test]
    fn test_state_serde() {
        let account_json: serde_json::Value = serde_json::from_slice(MARINADE_STATE_DATA).unwrap();
        let account_data = BASE64
            .decode(
                account_json["account"]["data"][0]
                    .as_str()
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();

        // Deserialization - skip first 8 bytes (account discriminator)
        let stake_pool = marinade_staking_sdk::State::borsh_de(&mut &account_data[..]).unwrap();
        println!("{:?}", stake_pool);

        let pause_authority = bs58::encode_pubkey(&stake_pool.pause_authority);
        println!("{:?}", pause_authority.str());
    }
}
