use proptest::prelude::*;
use sanctum_system_core::instructions::assign::{
    AssignIxAccs, AssignIxData, ASSIGN_IX_IS_SIGNER, ASSIGN_IX_IS_WRITABLE,
};
use solana_pubkey::Pubkey;
use solana_system_interface::instruction::assign;

use crate::common::{to_sol_ix, ID_PK};

type AssignIxKeys = AssignIxAccs<Pubkey>;

proptest! {
    #[test]
    fn check_assign_ix_against_sol(
        key: [u8; 32],
        owner: [u8; 32]
    ) {
        let [key, owner] = [key, owner].map(Pubkey::new_from_array);
        let sol = assign(&key, &owner);
        let us = to_sol_ix(
            &ID_PK,
            &AssignIxKeys::memset(ID_PK).with_assign(key).0,
            &ASSIGN_IX_IS_SIGNER.0,
            &ASSIGN_IX_IS_WRITABLE.0,
            AssignIxData::new(owner.as_array()).as_buf(),
        );
        prop_assert_eq!(sol, us);
    }
}
