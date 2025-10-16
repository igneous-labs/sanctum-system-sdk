use jiminy_cpi::{
    account::{AccountHandle, Accounts},
    pda::PdaSigner,
    program_error::ProgramError,
    AccountPerms, Cpi,
};
use sanctum_system_core::{
    instructions::assign::{
        AssignIxAccs, AssignIxData, ASSIGN_IX_ACCS_LEN, ASSIGN_IX_IS_SIGNER, ASSIGN_IX_IS_WRITABLE,
    },
    ID,
};

use super::{internal_utils::signer_writable_to_perms, SystemAccountHandlePerms};

pub type AssignIxAccounts<'a> = AssignIxAccs<AccountHandle<'a>>;
pub type AssignIxAccountPerms = AssignIxAccs<AccountPerms>;

pub const ASSIGN_IX_ACCOUNT_PERMS: AssignIxAccountPerms = AssignIxAccs(signer_writable_to_perms(
    ASSIGN_IX_IS_SIGNER.0,
    ASSIGN_IX_IS_WRITABLE.0,
));

#[inline]
pub fn assign_ix_account_handle_perms(
    a: AssignIxAccounts<'_>,
) -> SystemAccountHandlePerms<'_, ASSIGN_IX_ACCS_LEN> {
    a.0.into_iter().zip(ASSIGN_IX_ACCOUNT_PERMS.0)
}

/// System program must be in context
#[inline]
pub fn assign_invoke_fwd<'acc, const MAX_ACCOUNTS: usize, const MAX_CPI_ACCOUNTS: usize>(
    accounts: &mut Accounts<'acc, MAX_ACCOUNTS>,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: AssignIxAccounts<'acc>,
    owner: &[u8; 32],
) -> Result<(), ProgramError> {
    cpi.invoke_fwd(accounts, &ID, AssignIxData::new(owner).as_buf(), handles.0)
}

/// System program must be in context
#[inline]
pub fn assign_invoke_signed<'acc, const MAX_ACCOUNTS: usize, const MAX_CPI_ACCOUNTS: usize>(
    accounts: &mut Accounts<'acc, MAX_ACCOUNTS>,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: AssignIxAccounts<'acc>,
    owner: &[u8; 32],
    signers: &[PdaSigner],
) -> Result<(), ProgramError> {
    cpi.invoke_signed(
        accounts,
        &ID,
        AssignIxData::new(owner).as_buf(),
        assign_ix_account_handle_perms(handles),
        signers,
    )
}
