use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{to_bincode_discm, U64IxData};

use super::internal_utils::impl_memset;

// Accounts

#[generic_array_struct(builder pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferIxAccs<T> {
    pub from: T,
    pub to: T,
}

impl_memset!(TransferIxAccs);

pub type TransferIxAccsFlag = TransferIxAccs<bool>;

pub const TRANSFER_IX_IS_SIGNER: TransferIxAccsFlag =
    TransferIxAccsFlag::memset(false).const_with_from(true);

pub const TRANSFER_IX_IS_WRITABLE: TransferIxAccsFlag = TransferIxAccsFlag::memset(true);

// Data

const TRANSFER_IX_DISCM_RAW: u8 = 2;

pub const TRANSFER_IX_DISCM: [u8; 4] = to_bincode_discm(TRANSFER_IX_DISCM_RAW);

pub const TRANSFER_IX_DATA_LEN: usize = TransferIxData::LEN;

pub type TransferIxData = U64IxData<TRANSFER_IX_DISCM_RAW>;
