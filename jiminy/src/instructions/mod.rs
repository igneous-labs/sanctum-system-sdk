//! ## Dev Notes
//!
//! - allocate() is kinda pointless to implement for CPI because when you CPI allocate, you're
//!   limited to size=MAX_REALLOC_LENGTH. So you might as well assign account to yourself then realloc().

use core::{array, iter::Zip};

use jiminy_cpi::{account::AccountHandle, AccountPerms};

pub mod assign;
pub mod create_account;
pub mod transfer;

mod internal_utils;

pub type SystemInstr<'account, 'data, const ACCOUNTS: usize> = jiminy_cpi::Instr<
    'account,
    'data,
    Zip<
        array::IntoIter<AccountHandle<'account>, ACCOUNTS>,
        array::IntoIter<AccountPerms, ACCOUNTS>,
    >,
>;
