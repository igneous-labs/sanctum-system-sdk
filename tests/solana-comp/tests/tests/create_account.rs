use proptest::prelude::*;
use sanctum_system_core::instructions::create_account::{
    CreateAccountIxAccs, CreateAccountIxArgs, CreateAccountIxData, CREATE_ACCOUNT_IX_IS_SIGNER,
    CREATE_ACCOUNT_IX_IS_WRITABLE,
};
use solana_pubkey::Pubkey;
use solana_system_interface::instruction::create_account;

use crate::common::{to_sol_ix, ID_PK};

type CreateAccountIxKeys = CreateAccountIxAccs<Pubkey>;

proptest! {
    #[test]
    fn check_create_account_ix_against_sol(
        funding: [u8; 32],
        new: [u8; 32],
        lamports: u64,
        space: u64,
        owner: [u8; 32],
    ) {
        let [funding, new, owner] = [funding, new, owner].map(Pubkey::new_from_array);
        let sol = create_account(&funding, &new, lamports, space, &owner);
        let us = to_sol_ix(
            &ID_PK,
            &CreateAccountIxKeys::memset(ID_PK).with_funding(funding).with_new(new).0,
            &CREATE_ACCOUNT_IX_IS_SIGNER.0,
            &CREATE_ACCOUNT_IX_IS_WRITABLE.0,
            CreateAccountIxData::new(&CreateAccountIxArgs {
                lamports,
                space: space as usize,
                owner: owner.as_array(),
            }).as_buf(),
        );
        prop_assert_eq!(sol, us);
    }
}
