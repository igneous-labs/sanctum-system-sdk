use proptest::prelude::*;
use sanctum_system_core::instructions::transfer::{
    TransferIxAccs, TransferIxData, TRANSFER_IX_IS_SIGNER, TRANSFER_IX_IS_WRITABLE,
};
use sanctum_system_test_utils::to_sol_ix;
use solana_pubkey::Pubkey;
use solana_system_interface::instruction::transfer;

use crate::common::ID_PK;

type TransferIxKeys = TransferIxAccs<Pubkey>;

proptest! {
    #[test]
    fn check_transfer_ix_against_sol(
        from: [u8; 32],
        to: [u8; 32],
        lamports: u64,
    ) {
        let [from, to] = [from, to].map(Pubkey::new_from_array);
        let sol = transfer(&from, &to, lamports);
        let us = to_sol_ix(
            &ID_PK,
            &TransferIxKeys::memset(ID_PK).with_from(from).with_to(to).0,
            &TRANSFER_IX_IS_SIGNER.0,
            &TRANSFER_IX_IS_WRITABLE.0,
            TransferIxData::new(lamports).as_buf(),
        );
        prop_assert_eq!(sol, us);
    }
}
