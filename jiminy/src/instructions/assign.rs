use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_system_core::instructions::assign::{
    AssignIxAccs, AssignIxData, ASSIGN_IX_ACCS_LEN, ASSIGN_IX_IS_SIGNER, ASSIGN_IX_IS_WRITABLE,
};

use super::{internal_utils::signer_writable_to_perms, Instruction};

pub type AssignIxAccounts<'a> = AssignIxAccs<AccountHandle<'a>>;
pub type AssignIxAccountPerms = AssignIxAccs<AccountPerms>;

pub const ASSIGN_IX_ACCOUNT_PERMS: AssignIxAccountPerms = AssignIxAccs(signer_writable_to_perms(
    ASSIGN_IX_IS_SIGNER.0,
    ASSIGN_IX_IS_WRITABLE.0,
));

#[inline]
pub fn assign_ix<'account, 'data>(
    system_prog: AccountHandle<'account>,
    accounts: AssignIxAccounts<'account>,
    ix_data: &'data AssignIxData,
) -> Instruction<'account, 'data, ASSIGN_IX_ACCS_LEN> {
    Instruction {
        prog: system_prog,
        data: ix_data.as_buf(),
        accounts: accounts.0.into_iter().zip(ASSIGN_IX_ACCOUNT_PERMS.0),
    }
}
