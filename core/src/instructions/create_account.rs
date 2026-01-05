use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::caba;

use super::internal_utils::impl_memset;

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CreateAccountIxAccs<T> {
    pub funding: T,
    pub new: T,
}

impl_memset!(CreateAccountIxAccs);

pub type CreateAccountIxAccsFlag = CreateAccountIxAccs<bool>;

pub const CREATE_ACCOUNT_IX_IS_SIGNER: CreateAccountIxAccsFlag =
    CreateAccountIxAccsFlag::memset(true);

pub const CREATE_ACCOUNT_IX_IS_WRITABLE: CreateAccountIxAccsFlag =
    CreateAccountIxAccsFlag::memset(true);

// Data

pub const CREATE_ACCOUNT_IX_DISCM: [u8; 4] = [0; 4];

pub const CREATE_ACCOUNT_IX_DATA_LEN: usize = 52;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CreateAccountIxData([u8; CREATE_ACCOUNT_IX_DATA_LEN]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreateAccountIxArgs<'a> {
    pub lamports: u64,
    pub space: usize,
    pub owner: &'a [u8; 32],
}

impl CreateAccountIxData {
    #[inline]
    pub const fn new(
        CreateAccountIxArgs {
            lamports,
            space,
            owner,
        }: &CreateAccountIxArgs,
    ) -> Self {
        const A: usize = CREATE_ACCOUNT_IX_DATA_LEN;

        let mut d = [0u8; A];

        d = caba::<A, 0, 4>(d, &CREATE_ACCOUNT_IX_DISCM);
        d = caba::<A, 4, 8>(d, &lamports.to_le_bytes());
        d = caba::<A, 12, 8>(d, &(*space as u64).to_le_bytes());
        d = caba::<A, 20, 32>(d, owner);

        Self(d)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; CREATE_ACCOUNT_IX_DATA_LEN] {
        &self.0
    }
}
