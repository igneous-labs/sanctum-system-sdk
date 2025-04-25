use solana_account::Account;
use solana_pubkey::Pubkey;
use solana_rent::Rent;

/// asserts that the result of executing a instruction or sequence of instructions on mollusk
/// will not result in a `TransactionError`
pub fn assert_valid_tx(pre: &[(Pubkey, Account)], post: &[(Pubkey, Account)]) {
    assert_balanced_tx(pre, post);
    assert_all_accounts_rent_exempt(post);
}

fn assert_balanced_tx(pre: &[(Pubkey, Account)], post: &[(Pubkey, Account)]) {
    let sum_lamports = |acc, (_pk, account): &(Pubkey, Account)| acc + account.lamports;
    let [pre_lamports, post_lamports] = [pre, post].map(|slice| slice.iter().fold(0, sum_lamports));
    assert_eq!(pre_lamports, post_lamports);
}

fn assert_all_accounts_rent_exempt(post: &[(Pubkey, Account)]) {
    let rent = Rent::default();
    post.iter()
        .for_each(|(_pk, acc)| assert!(acc.lamports >= rent.minimum_balance(acc.data.len())));
}
