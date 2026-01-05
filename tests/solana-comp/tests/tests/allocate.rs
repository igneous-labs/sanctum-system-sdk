use proptest::prelude::*;
use sanctum_system_core::instructions::allocate::{
    AllocateIxData, NewAllocateIxAccsBuilder, ALLOCATE_IX_IS_SIGNER, ALLOCATE_IX_IS_WRITABLE,
};
use sanctum_system_test_utils::to_sol_ix;
use solana_pubkey::Pubkey;
use solana_system_interface::instruction::allocate;

use crate::common::ID_PK;

proptest! {
    #[test]
    fn check_allocate_ix_against_sol(
        alloc: [u8; 32],
        space: u64,
    ) {
        let alloc = Pubkey::new_from_array(alloc);
        let sol = allocate(&alloc, space);
        let us = to_sol_ix(
            &ID_PK,
            &NewAllocateIxAccsBuilder::start().with_allocate(alloc).build().0,
            &ALLOCATE_IX_IS_SIGNER.0,
            &ALLOCATE_IX_IS_WRITABLE.0,
            AllocateIxData::new(space).as_buf(),
        );
        prop_assert_eq!(sol, us);
    }
}
