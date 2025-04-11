use mollusk_svm::Mollusk;
use sanctum_marinade_liquid_staking_core::{self as marinade_staking_sdk};
use solana_pubkey::Pubkey;

use super::{
    test_fixtures_dir, BPF_LOADER_UPGRADEABLE_PROGRAM_ID, MIN_1_SOL_DELEGATION_FEATURE_ID,
};

pub fn mollusk_marinade_prog() -> Mollusk {
    let mut res = Mollusk::default();
    res.add_program_with_elf_and_loader(
        &Pubkey::new_from_array(marinade_staking_sdk::MARINADE_STAKING_PROGRAM),
        &std::fs::read(test_fixtures_dir().join("marinade-staking.so")).unwrap(),
        &Pubkey::new_from_array(BPF_LOADER_UPGRADEABLE_PROGRAM_ID),
    );
    warp_to_epoch(&mut res, 760);
    res.feature_set
        .deactivate(&Pubkey::new_from_array(MIN_1_SOL_DELEGATION_FEATURE_ID));
    mollusk_svm_programs_token::token::add_program(&mut res);
    res
}

pub fn warp_to_epoch(mollusk: &mut Mollusk, epoch: u64) {
    mollusk.sysvars.warp_to_slot(432_000 * epoch + 69);
}
