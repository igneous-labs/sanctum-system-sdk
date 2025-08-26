//! This program creates an account with itself as owner
//! by CPI-ing system program allocate, transfer rent-exempt lamports,
//! then realloc-ing the account to desired size.
//!
//! Args:
//! - `size: u64` data size of the account to create

#![allow(unexpected_cfgs)]

use jiminy_entrypoint::program_error::{BuiltInProgramError, ProgramError};
use jiminy_sysvar_rent::{sysvar::SimpleSysvar, Rent};
use sanctum_system_jiminy::{
    instructions::{
        assign::assign_ix_account_handle_perms, transfer::transfer_ix_account_handle_perms,
    },
    sanctum_system_core::instructions::{
        assign::{AssignIxData, NewAssignIxAccsBuilder},
        transfer::{NewTransferIxAccsBuilder, TransferIxData},
    },
};

const MAX_ACCOUNTS: usize = 3;

type Accounts<'a> = jiminy_entrypoint::account::Accounts<'a, MAX_ACCOUNTS>;
type Cpi = jiminy_cpi::Cpi<3>;

jiminy_entrypoint::entrypoint!(process_ix, MAX_ACCOUNTS);

fn process_ix(
    accounts: &mut Accounts,
    data: &[u8],
    prog_id: &[u8; 32],
) -> Result<(), ProgramError> {
    let [sys_prog, from, to] = accounts.as_slice() else {
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

    let mut cpi = Cpi::new();
    let sys_prog_key = *accounts.get(sys_prog).key();

    cpi.invoke_signed(
        accounts,
        &sys_prog_key,
        AssignIxData::new(prog_id).as_buf(),
        assign_ix_account_handle_perms(NewAssignIxAccsBuilder::start().with_assign(to).build()),
        &[],
    )?;
    cpi.invoke_signed(
        accounts,
        &sys_prog_key,
        TransferIxData::new(lamports).as_buf(),
        transfer_ix_account_handle_perms(
            NewTransferIxAccsBuilder::start()
                .with_from(from)
                .with_to(to)
                .build(),
        ),
        &[],
    )?;

    accounts.get_mut(to).realloc(space, false)
}
