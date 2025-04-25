use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_system_core::instructions::create_account::{
    CreateAccountIxAccs, CreateAccountIxData, CREATE_ACCOUNT_IX_ACCS_LEN,
    CREATE_ACCOUNT_IX_IS_SIGNER, CREATE_ACCOUNT_IX_IS_WRITABLE,
};

use super::{internal_utils::signer_writable_to_perms, SystemInstr};

pub type CreateAccountIxAccounts<'a> = CreateAccountIxAccs<AccountHandle<'a>>;
pub type CreateAccountIxAccountPerms = CreateAccountIxAccs<AccountPerms>;

pub const CREATE_ACCOUNT_IX_ACCOUNT_PERMS: CreateAccountIxAccountPerms =
    CreateAccountIxAccs(signer_writable_to_perms(
        CREATE_ACCOUNT_IX_IS_SIGNER.0,
        CREATE_ACCOUNT_IX_IS_WRITABLE.0,
    ));

#[inline]
pub fn create_account_ix<'account, 'data>(
    system_prog: AccountHandle<'account>,
    accounts: CreateAccountIxAccounts<'account>,
    ix_data: &'data CreateAccountIxData,
) -> SystemInstr<'account, 'data, CREATE_ACCOUNT_IX_ACCS_LEN> {
    SystemInstr {
        prog: system_prog,
        data: ix_data.as_buf(),
        accounts: accounts
            .0
            .into_iter()
            .zip(CREATE_ACCOUNT_IX_ACCOUNT_PERMS.0),
    }
}
