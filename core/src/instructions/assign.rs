use generic_array_struct::generic_array_struct;

use super::internal_utils::{caba, impl_memset};

// Accounts

#[generic_array_struct(pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AssignIxAccs<T> {
    pub assign: T,
}

impl<T> AssignIxAccs<T> {
    impl_memset!(ASSIGN_IX_ACCS_LEN);
}

pub type AssignIxAccsFlag = AssignIxAccs<bool>;

pub const ASSIGN_IX_IS_SIGNER: AssignIxAccsFlag = AssignIxAccsFlag::memset(true);

pub const ASSIGN_IX_IS_WRITABLE: AssignIxAccsFlag = AssignIxAccsFlag::memset(true);

// Data

pub const ASSIGN_IX_DISCM: [u8; 4] = [1, 0, 0, 0];

pub const ASSIGN_IX_DATA_LEN: usize = 36;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AssignIxData([u8; ASSIGN_IX_DATA_LEN]);

impl AssignIxData {
    #[inline]
    pub const fn new(owner: &[u8; 32]) -> Self {
        const A: usize = ASSIGN_IX_DATA_LEN;

        let mut ix_data = [0u8; A];
        ix_data = caba::<A, 0, 4>(ix_data, &ASSIGN_IX_DISCM);
        ix_data = caba::<A, 4, 32>(ix_data, owner);

        Self(ix_data)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; ASSIGN_IX_DATA_LEN] {
        &self.0
    }
}
