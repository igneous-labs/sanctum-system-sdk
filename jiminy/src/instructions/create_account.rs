use jiminy_cpi::{
    account::{Abr, AccountHandle},
    pda::PdaSigner,
    program_error::ProgramError,
    AccountPerms, Cpi,
};
use sanctum_system_core::{
    instructions::create_account::{
        CreateAccountIxAccs, CreateAccountIxArgs, CreateAccountIxData, CREATE_ACCOUNT_IX_ACCS_LEN,
        CREATE_ACCOUNT_IX_IS_SIGNER, CREATE_ACCOUNT_IX_IS_WRITABLE,
    },
    ID,
};

use super::{internal_utils::signer_writable_to_perms, SystemAccountHandlePerms};

pub type CreateAccountIxAccounts<'a> = CreateAccountIxAccs<AccountHandle<'a>>;
pub type CreateAccountIxAccountPerms = CreateAccountIxAccs<AccountPerms>;

pub const CREATE_ACCOUNT_IX_ACCOUNT_PERMS: CreateAccountIxAccountPerms =
    CreateAccountIxAccs(signer_writable_to_perms(
        CREATE_ACCOUNT_IX_IS_SIGNER.0,
        CREATE_ACCOUNT_IX_IS_WRITABLE.0,
    ));

#[inline]
pub fn create_account_ix_account_handle_perms(
    a: CreateAccountIxAccounts<'_>,
) -> SystemAccountHandlePerms<'_, CREATE_ACCOUNT_IX_ACCS_LEN> {
    a.0.into_iter().zip(CREATE_ACCOUNT_IX_ACCOUNT_PERMS.0)
}

/// System program must be in context
#[inline]
pub fn create_account_invoke_fwd<'acc, const MAX_CPI_ACCOUNTS: usize>(
    abr: &mut Abr,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: CreateAccountIxAccounts<'acc>,
    args: &CreateAccountIxArgs<'_>,
) -> Result<(), ProgramError> {
    cpi.invoke_fwd(abr, &ID, CreateAccountIxData::new(args).as_buf(), handles.0)
}

/// System program must be in context
#[inline]
pub fn create_account_invoke_signed<'acc, const MAX_CPI_ACCOUNTS: usize>(
    abr: &mut Abr,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: CreateAccountIxAccounts<'acc>,
    args: &CreateAccountIxArgs<'_>,
    signers: &[PdaSigner],
) -> Result<(), ProgramError> {
    cpi.invoke_signed(
        abr,
        &ID,
        CreateAccountIxData::new(args).as_buf(),
        create_account_ix_account_handle_perms(handles),
        signers,
    )
}
