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

        assert_eq!(stake_pool.msol_supply, 3597210656032211);
        assert_eq!(stake_pool.available_reserve_balance, 265139147340070);

        assert_eq!(
            bs58::encode_pubkey(&stake_pool.pause_authority).str(),
            "AjGjLWx7vbzgPNxPSQUPjLNjeavQCHVS9VoJNWpnyP6n"
        );

        assert_eq!(
            bs58::encode_pubkey(&stake_pool.msol_mint).str(),
            "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"
        );

        assert_eq!(
            bs58::encode_pubkey(&stake_pool.admin_authority).str(),
            "42VJbDihcS81YJPbuhHnHgvo1ehu42j8VK9sNwrnAarR"
        );

        assert_eq!(
            bs58::encode_pubkey(&stake_pool.operational_sol_account).str(),
            "opLSF7LdfyWNBby5o6FT8UFsr2A4UGKteECgtLSYrSm"
        );

        assert_eq!(
            bs58::encode_pubkey(&stake_pool.treasury_msol_account).str(),
            "B1aLzaNMeFVAyQ6f3XbbUyKcH2YPHu2fqiEagmiF23VR"
        );
    }
}
