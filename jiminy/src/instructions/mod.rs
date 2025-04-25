use core::{array, iter::Zip};

use jiminy_cpi::{account::AccountHandle, AccountPerms};

pub mod assign;

mod internal_utils;

pub type Instruction<'account, 'data, const ACCOUNTS: usize> = jiminy_cpi::Instr<
    'account,
    'data,
    Zip<
        array::IntoIter<AccountHandle<'account>, ACCOUNTS>,
        array::IntoIter<AccountPerms, ACCOUNTS>,
    >,
>;
