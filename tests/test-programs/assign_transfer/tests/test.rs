//! .so file size 7176
//!
//! Create is more efficient in both binary size and CUs than Assign + transfer + realloc.
//! CPI is extremely compute intensive.
//!
//! Each call to invoke() costs 1K CUs min after all:
//! https://github.com/anza-xyz/agave/blob/fd207f94823c0193bc87fdbe200378c48c19ee04/program-runtime/src/execution_budget.rs#L167

#![cfg(feature = "test-sbf")]

use jiminy_cpi::account::MAX_PERMITTED_DATA_INCREASE;
use mollusk_svm::{
    program::keyed_account_for_system_program,
    result::{Check, InstructionResult},
    Mollusk,
};
use proptest::prelude::*;
use sanctum_system_jiminy::sanctum_system_core::ID;
use sanctum_system_test_utils::{
    is_tx_balanced, save_cus_to_file, silence_mollusk_prog_logs, two_diff_pks,
};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

const PROG_NAME: &str = "assign_transfer_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("2uX9abtfRANMnpTMbXQBAxn4vDMp6ok4c1jyBw8vQPoF");

thread_local! {
    static SVM: Mollusk = Mollusk::new(&PROG_ID, PROG_NAME);
}

const TO_ACC_IDX: usize = 2;

#[test]
fn assign_transfer_cus() {
    const FROM: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
    const TO: Pubkey = solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");
    const SIZE: usize = 888;

    let accounts = ix_accounts(FROM, TO);
    let instr = ix(FROM, TO, SIZE);

    SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        let to = &resulting_accounts[TO_ACC_IDX].1;
        assert_eq!(to.owner, PROG_ID);
        assert_eq!(to.data.len(), SIZE);

        save_cus_to_file("basic", compute_units_consumed);
    });
}

proptest! {
    #[test]
    fn assign_transfer_all(
        pks in two_diff_pks(),
        size in 0..=MAX_PERMITTED_DATA_INCREASE,
    ) {
        silence_mollusk_prog_logs();

        let [from, to] = pks.map(Pubkey::new_from_array);

        let accounts = ix_accounts(from, to);
        let instr = ix(from, to, size);

        SVM.with(|svm| {
            let InstructionResult {
                raw_result,
                resulting_accounts,
                ..
            } = svm.process_and_validate_instruction(
                &instr,
                &accounts,
                &[Check::all_rent_exempt()],
            );

            raw_result.unwrap();

            prop_assert!(is_tx_balanced(&accounts, &resulting_accounts));

            let to = &resulting_accounts[TO_ACC_IDX].1;
            prop_assert_eq!(to.owner, PROG_ID);
            prop_assert_eq!(to.data.len(), size);

            Ok(())
        }).unwrap();
    }
}

fn from_to_accs(from: Pubkey, to: Pubkey) -> [(Pubkey, Account); 2] {
    [
        (
            from,
            Account {
                // have enough to pay for all possible rent-exemptions
                // while not triggering overflows
                lamports: u64::MAX / 2,
                ..Default::default()
            },
        ),
        (to, Account::default()), // empty account
    ]
}

fn ix_accounts(from: Pubkey, to: Pubkey) -> [(Pubkey, Account); 3] {
    let [from, to] = from_to_accs(from, to);
    [keyed_account_for_system_program(), from, to]
}

fn ix(from: Pubkey, to: Pubkey, size: usize) -> Instruction {
    Instruction {
        program_id: PROG_ID,
        accounts: [
            AccountMeta {
                pubkey: Pubkey::new_from_array(ID),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: from,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: to,
                is_signer: true,
                is_writable: true,
            },
        ]
        .into(),
        data: (size as u64).to_le_bytes().into(),
    }
}
