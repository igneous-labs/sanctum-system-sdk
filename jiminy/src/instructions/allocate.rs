use jiminy_cpi::{
    account::{Abr, AccountHandle},
    pda::PdaSigner,
    program_error::ProgramError,
    AccountPerms, Cpi,
};
use sanctum_system_core::{
    instructions::allocate::{
        AllocateIxAccs, AllocateIxData, ALLOCATE_IX_ACCS_LEN, ALLOCATE_IX_IS_SIGNER,
        ALLOCATE_IX_IS_WRITABLE,
    },
    ID,
};

use super::{internal_utils::signer_writable_to_perms, SystemAccountHandlePerms};

pub type AllocateIxAccounts<'a> = AllocateIxAccs<AccountHandle<'a>>;
pub type AllocateIxAccountPerms = AllocateIxAccs<AccountPerms>;

pub const ALLOCATE_IX_ACCOUNT_PERMS: AllocateIxAccountPerms = AllocateIxAccs(
    signer_writable_to_perms(ALLOCATE_IX_IS_SIGNER.0, ALLOCATE_IX_IS_WRITABLE.0),
);

#[inline]
pub fn allocate_ix_account_handle_perms(
    a: AllocateIxAccounts<'_>,
) -> SystemAccountHandlePerms<'_, ALLOCATE_IX_ACCS_LEN> {
    a.0.into_iter().zip(ALLOCATE_IX_ACCOUNT_PERMS.0)
}

/// System program must be in context
#[inline]
pub fn allocate_invoke_fwd<'acc, const MAX_CPI_ACCOUNTS: usize>(
    abr: &mut Abr,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: AllocateIxAccounts<'acc>,
    space: u64,
) -> Result<(), ProgramError> {
    cpi.invoke_fwd(abr, &ID, AllocateIxData::new(space).as_buf(), handles.0)
}

/// System program must be in context
#[inline]
pub fn allocate_invoke_signed<'acc, const MAX_CPI_ACCOUNTS: usize>(
    abr: &mut Abr,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: AllocateIxAccounts<'acc>,
    space: u64,
    signers: &[PdaSigner],
) -> Result<(), ProgramError> {
    cpi.invoke_signed(
        abr,
        &ID,
        AllocateIxData::new(space).as_buf(),
        allocate_ix_account_handle_perms(handles),
        signers,
    )
}
