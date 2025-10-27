//! This program creates an account with itself as owner
//! by CPI-ing system program create account
//!
//! Args:
//! - `size: u64` data size of the account to create

#![allow(unexpected_cfgs)]

use jiminy_cpi::account::{Abr, AccountHandle};
use jiminy_entrypoint::program_error::{BuiltInProgramError, ProgramError};
use jiminy_sysvar_rent::{sysvar::SimpleSysvar, Rent};
use sanctum_system_jiminy::{
    instructions::create_account::create_account_ix_account_handle_perms,
    sanctum_system_core::instructions::create_account::{
        CreateAccountIxArgs, CreateAccountIxData, NewCreateAccountIxAccsBuilder,
    },
};

const MAX_ACCOUNTS: usize = 3;

type Cpi = jiminy_cpi::Cpi<3>;

jiminy_entrypoint::entrypoint!(process_ix, MAX_ACCOUNTS);

fn process_ix(
    abr: &mut Abr,
    accounts: &[AccountHandle<'_>],
    data: &[u8],
    prog_id: &[u8; 32],
) -> Result<(), ProgramError> {
    let [sys_prog, from, to] = accounts else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [sys_prog, from, to] = [sys_prog, from, to].map(|h| *h);

    let space = u64::from_le_bytes(
        *<&[u8; 8]>::try_from(data)
            .map_err(|_| ProgramError::from_builtin(BuiltInProgramError::InvalidInstructionData))?,
    ) as usize;

    let lamports = Rent::get()?.min_balance(space);
    let sys_prog_key = *abr.get(sys_prog).key();

    Cpi::new().invoke_signed(
        abr,
        &sys_prog_key,
        CreateAccountIxData::new(&CreateAccountIxArgs {
            lamports,
            space,
            owner: prog_id,
        })
        .as_buf(),
        create_account_ix_account_handle_perms(
            NewCreateAccountIxAccsBuilder::start()
                .with_funding(from)
                .with_new(to)
                .build(),
        ),
        &[],
    )
}
