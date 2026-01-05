//! ## Dev Notes
//!
//! - allocate() is kinda pointless to implement for CPI because when you CPI allocate, you're
//!   limited to size=MAX_REALLOC_LENGTH. So you might as well assign account to yourself then realloc().

use core::{array, iter::Zip};

use jiminy_cpi::{account::AccountHandle, AccountPerms};

pub mod allocate;
pub mod assign;
pub mod create_account;
pub mod transfer;

mod internal_utils;

/// `impls IntoIterator<Item = (AccountHandle, AccountPerms)>`
pub type SystemAccountHandlePerms<'account, const ACCOUNTS: usize> = Zip<
    array::IntoIter<AccountHandle<'account>, ACCOUNTS>,
    array::IntoIter<AccountPerms, ACCOUNTS>,
>;
