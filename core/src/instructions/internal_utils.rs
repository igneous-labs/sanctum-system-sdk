macro_rules! impl_memset {
    ($Gas:ident) => {
        impl<T: Copy> $Gas<T> {
            #[inline]
            pub const fn memset(val: T) -> Self {
                Self([val; _])
            }
        }
    };
}

pub(crate) use impl_memset;

// This does not seem to produce different bytecode
// on-chain compared to .copy_from_slice(), but it allows us to retain `const`
/// caba = `const_assign_byte_array`
pub(crate) const fn caba<const A: usize, const START: usize, const LEN: usize>(
    mut arr: [u8; A],
    val: &[u8; LEN],
) -> [u8; A] {
    const {
        assert!(START + LEN <= A);
    }

    let mut i = 0;
    while i < LEN {
        arr[START + i] = val[i];
        i += 1;
    }
    arr
}

/// csba = `const_split_byte_array`
#[inline]
pub(crate) const fn csba<const M: usize, const N: usize, const X: usize>(
    data: &[u8; M],
) -> (&[u8; N], &[u8; X]) {
    const {
        assert!(N <= M);
        assert!(X == M - N)
    }

    // Safety: bounds checked above
    let (a, b) = unsafe { data.split_at_unchecked(N) };

    // SAFETY: data is guaranteed to be of length M
    // and we are splitting it into two slices of length N and X (i.e M-N)
    (unsafe { &*a.as_ptr().cast::<[u8; N]>() }, unsafe {
        &*b.as_ptr().cast::<[u8; X]>()
    })
}

/// Returns `None` if discm does not match, Some(rest of data) otherwise
#[inline]
pub(crate) const fn discm_checked<const M: usize, const D: usize>(
    expected_discm: [u8; 4],
    data: &[u8; M],
) -> Option<&[u8; D]> {
    let (discm, data) = csba::<M, 4, D>(data);

    let mut i = 0;
    while i < 4 {
        if discm[i] != expected_discm[i] {
            return None;
        }
        i += 1;
    }

    Some(data)
}

#[inline]
pub(crate) const fn to_bincode_discm(discm: u8) -> [u8; 4] {
    [discm, 0, 0, 0]
}

pub const U64_IX_DATA_LEN: usize = 12;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U64IxData<const DISCM_RAW: u8>([u8; U64_IX_DATA_LEN]);

impl<const DISCM_RAW: u8> U64IxData<DISCM_RAW> {
    pub const LEN: usize = U64_IX_DATA_LEN;

    #[inline]
    pub const fn new(arg: u64) -> Self {
        const A: usize = U64_IX_DATA_LEN;

        let mut res = [0; A];

        res = caba::<A, 0, 4>(res, &to_bincode_discm(DISCM_RAW));
        res = caba::<A, 4, 8>(res, &arg.to_le_bytes());

        Self(res)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; U64_IX_DATA_LEN] {
        &self.0
    }

    #[inline]
    pub const fn parse_no_discm(data: &[u8; 8]) -> u64 {
        u64::from_le_bytes(*data)
    }

    /// Returns `None` if discm does not match
    #[inline]
    pub const fn parse(data: &[u8; U64_IX_DATA_LEN]) -> Option<u64> {
        match discm_checked(to_bincode_discm(DISCM_RAW), data) {
            None => None,
            Some(d) => Some(Self::parse_no_discm(d)),
        }
    }
}
