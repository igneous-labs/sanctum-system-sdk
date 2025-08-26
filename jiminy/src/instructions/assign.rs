use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_system_core::instructions::assign::{
    AssignIxAccs, ASSIGN_IX_ACCS_LEN, ASSIGN_IX_IS_SIGNER, ASSIGN_IX_IS_WRITABLE,
};

use super::{internal_utils::signer_writable_to_perms, SystemAccountHandlePerms};

pub type AssignIxAccounts<'a> = AssignIxAccs<AccountHandle<'a>>;
pub type AssignIxAccountPerms = AssignIxAccs<AccountPerms>;

pub const ASSIGN_IX_ACCOUNT_PERMS: AssignIxAccountPerms = AssignIxAccs(signer_writable_to_perms(
    ASSIGN_IX_IS_SIGNER.0,
    ASSIGN_IX_IS_WRITABLE.0,
));

pub fn assign_ix_account_handle_perms(
    a: AssignIxAccounts,
) -> SystemAccountHandlePerms<'_, ASSIGN_IX_ACCS_LEN> {
    a.0.into_iter().zip(ASSIGN_IX_ACCOUNT_PERMS.0)
}
