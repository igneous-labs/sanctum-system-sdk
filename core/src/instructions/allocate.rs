// Accounts

use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, to_bincode_discm, U64IxData};

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AllocateIxAccs<T> {
    pub allocate: T,
}

impl_memset!(AllocateIxAccs);

pub type AllocateIxAccsFlag = AllocateIxAccs<bool>;

pub const ALLOCATE_IX_IS_SIGNER: AllocateIxAccsFlag = AllocateIxAccsFlag::memset(true);

pub const ALLOCATE_IX_IS_WRITABLE: AllocateIxAccsFlag = AllocateIxAccsFlag::memset(true);

// Data

const ALLOCATE_IX_DISCM_RAW: u8 = 8;

pub const ALLOCATE_IX_DISCM: [u8; 4] = to_bincode_discm(ALLOCATE_IX_DISCM_RAW);

pub const ALLOCATE_IX_DATA_LEN: usize = AllocateIxData::LEN;

pub type AllocateIxData = U64IxData<ALLOCATE_IX_DISCM_RAW>;
