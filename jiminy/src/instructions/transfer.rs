use jiminy_cpi::{
    account::{Abr, AccountHandle},
    pda::PdaSigner,
    program_error::ProgramError,
    AccountPerms, Cpi,
};
use sanctum_system_core::{
    instructions::transfer::{
        TransferIxAccs, TransferIxData, TRANSFER_IX_ACCS_LEN, TRANSFER_IX_IS_SIGNER,
        TRANSFER_IX_IS_WRITABLE,
    },
    ID,
};

use super::{internal_utils::signer_writable_to_perms, SystemAccountHandlePerms};

pub type TransferIxAccounts<'a> = TransferIxAccs<AccountHandle<'a>>;
pub type TransferIxAccountPerms = TransferIxAccs<AccountPerms>;

pub const TRANSFER_IX_ACCOUNT_PERMS: TransferIxAccountPerms = TransferIxAccs(
    signer_writable_to_perms(TRANSFER_IX_IS_SIGNER.0, TRANSFER_IX_IS_WRITABLE.0),
);

#[inline]
pub fn transfer_ix_account_handle_perms(
    a: TransferIxAccounts<'_>,
) -> SystemAccountHandlePerms<'_, TRANSFER_IX_ACCS_LEN> {
    a.0.into_iter().zip(TRANSFER_IX_ACCOUNT_PERMS.0)
}

/// System program must be in context
#[inline]
pub fn transfer_invoke_fwd<'acc, const MAX_CPI_ACCOUNTS: usize>(
    abr: &mut Abr,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: TransferIxAccounts<'acc>,
    lamports: u64,
) -> Result<(), ProgramError> {
    cpi.invoke_fwd(abr, &ID, TransferIxData::new(lamports).as_buf(), handles.0)
}

/// System program must be in context
#[inline]
pub fn transfer_invoke_signed<'acc, const MAX_CPI_ACCOUNTS: usize>(
    abr: &mut Abr,
    cpi: &mut Cpi<MAX_CPI_ACCOUNTS>,
    handles: TransferIxAccounts<'acc>,
    lamports: u64,
    signers: &[PdaSigner],
) -> Result<(), ProgramError> {
    cpi.invoke_signed(
        abr,
        &ID,
        TransferIxData::new(lamports).as_buf(),
        transfer_ix_account_handle_perms(handles),
        signers,
    )
}
