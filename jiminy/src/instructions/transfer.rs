use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_system_core::instructions::transfer::{
    TransferIxAccs, TransferIxData, TRANSFER_IX_ACCS_LEN, TRANSFER_IX_IS_SIGNER,
    TRANSFER_IX_IS_WRITABLE,
};

use super::{internal_utils::signer_writable_to_perms, SystemInstr};

pub type TransferIxAccounts<'a> = TransferIxAccs<AccountHandle<'a>>;
pub type TransferIxAccountPerms = TransferIxAccs<AccountPerms>;

pub const TRANSFER_IX_ACCOUNT_PERMS: TransferIxAccountPerms = TransferIxAccs(
    signer_writable_to_perms(TRANSFER_IX_IS_SIGNER.0, TRANSFER_IX_IS_WRITABLE.0),
);

#[inline]
pub fn transfer_ix<'account, 'data>(
    system_prog: AccountHandle<'account>,
    accounts: TransferIxAccounts<'account>,
    ix_data: &'data TransferIxData,
) -> SystemInstr<'account, 'data, TRANSFER_IX_ACCS_LEN> {
    SystemInstr {
        prog: system_prog,
        data: ix_data.as_buf(),
        accounts: accounts.0.into_iter().zip(TRANSFER_IX_ACCOUNT_PERMS.0),
    }
}
