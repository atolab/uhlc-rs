//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
use alloc::{
    format,
    string::{String, ToString},
};
use core::{
    convert::{TryFrom, TryInto},
    fmt,
    hash::Hash,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8},
    str::FromStr,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// An identifier for an HLC ([MAX_SIZE](ID::MAX_SIZE) bytes maximum).
/// This struct has a constant memory size (holding internally a `NonZeroU8`),
/// allowing allocations on the stack for better performances.
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use uhlc::ID;
///
/// let buf = [0x1a, 0x2b, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00,
///            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
/// // NOTE: ID::try_from(slice: &[u8]) assumes the slice is in little endian
/// let id1 = ID::try_from(&buf[..3]).unwrap();
/// assert_eq!(id1.size(), 3);
/// assert_eq!(id1.to_le_bytes(), buf);
/// assert_eq!(&id1.to_le_bytes()[..id1.size()], &[0x1a, 0x2b, 0x3c]);
/// let id2: ID = "3c2b1a".parse().unwrap();
/// assert_eq!(id2.size(), 3);
/// assert_eq!(id2.to_le_bytes(), buf);
/// assert_eq!(&id2.to_le_bytes()[..id2.size()], &[0x1a, 0x2b, 0x3c]);
/// assert_eq!(id2.to_string(), "3c2b1a");
/// assert_eq!(id1, id2);
/// ```
///
/// ```
/// use uhlc::ID;
///
/// let id = ID::rand();
/// assert!(id.size() <= 16);
/// ```
#[derive(Copy, Clone, Eq, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(transparent)]
pub struct ID([u8; ID::MAX_SIZE]);

impl ID {
    /// The maximum size of an le-encoded [`ID`](`ID`) in bytes: 16.
    pub const MAX_SIZE: usize = u128::BITS as usize / 8;

    /// The size of this [`ID`](`ID`) in bytes. I.e., the number of significant bytes of the le-encoded [`ID`](`ID`).
    #[inline]
    pub fn size(&self) -> usize {
        Self::MAX_SIZE - (u128::from_le_bytes(self.0).leading_zeros() as usize / 8)
    }

    /// This ID as bytes
    ///
    /// If you want to retrive a le-encoded slice of the [`ID`](`ID`), you can do it as follows:
    /// ```
    /// use uhlc::ID;
    /// use std::convert::TryFrom;
    ///
    /// let id = ID::try_from(&[0x01]).unwrap();
    /// let slice = &id.to_le_bytes()[..id.size()];
    /// assert_eq!(1, slice.len());
    /// assert_eq!(&[0x01], slice);
    /// ```
    #[inline]
    pub fn to_le_bytes(&self) -> [u8; Self::MAX_SIZE] {
        self.0
    }

    /// Generate a random [`ID`](`ID`).
    #[inline]
    pub fn rand() -> Self {
        use rand::rngs::OsRng;
        let id: u128 = OsRng.gen_range(1..u128::MAX);
        Self(id.to_le_bytes())
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SizeError(pub usize);
impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Maximum ID size ({} bytes) exceeded: {}",
            ID::MAX_SIZE,
            self.0
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SizeError {}

macro_rules! impl_from_sized_slice_for_id {
    ($N: expr) => {
        impl TryFrom<&[u8; $N]> for ID {
            type Error = SizeError;

            /// Performs the conversion.
            /// NOTE: the bytes slice is interpreted as little endian
            fn try_from(value: &[u8; $N]) -> Result<Self, Self::Error> {
                let mut id = [0u8; ID::MAX_SIZE];
                id[..$N].copy_from_slice(value);
                let id = u128::from_le_bytes(id);
                match NonZeroU128::new(id) {
                    Some(_) => Ok(Self(id.to_le_bytes())),
                    None => Err(SizeError(0)),
                }
            }
        }

        impl TryFrom<[u8; $N]> for ID {
            type Error = SizeError;

            /// Performs the conversion.
            /// NOTE: the bytes slice is interpreted as little endian
            fn try_from(id: [u8; $N]) -> Result<Self, Self::Error> {
                (&id).try_into()
            }
        }
    };
}
impl_from_sized_slice_for_id!(1);
impl_from_sized_slice_for_id!(2);
impl_from_sized_slice_for_id!(3);
impl_from_sized_slice_for_id!(4);
impl_from_sized_slice_for_id!(5);
impl_from_sized_slice_for_id!(6);
impl_from_sized_slice_for_id!(7);
impl_from_sized_slice_for_id!(8);
impl_from_sized_slice_for_id!(9);
impl_from_sized_slice_for_id!(10);
impl_from_sized_slice_for_id!(11);
impl_from_sized_slice_for_id!(12);
impl_from_sized_slice_for_id!(13);
impl_from_sized_slice_for_id!(14);
impl_from_sized_slice_for_id!(15);
impl_from_sized_slice_for_id!(16);

impl TryFrom<&[u8]> for ID {
    type Error = SizeError;

    /// Performs the conversion.  
    /// NOTE: the bytes slice is interpreted as little endian
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let size = slice.len();
        if size > Self::MAX_SIZE {
            return Err(SizeError(size));
        }
        let mut id = [0u8; ID::MAX_SIZE];
        id[..size].copy_from_slice(slice);
        let id = u128::from_le_bytes(id);
        match NonZeroU128::new(id) {
            Some(_) => Ok(Self(id.to_le_bytes())),
            None => Err(SizeError(0)),
        }
    }
}

impl TryFrom<u8> for ID {
    type Error = SizeError;

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        id.to_le_bytes().try_into()
    }
}

impl From<NonZeroU8> for ID {
    fn from(id: NonZeroU8) -> Self {
        NonZeroU128::from(id).into()
    }
}

impl TryFrom<u16> for ID {
    type Error = SizeError;

    fn try_from(id: u16) -> Result<Self, Self::Error> {
        id.to_le_bytes().try_into()
    }
}

impl From<NonZeroU16> for ID {
    fn from(id: NonZeroU16) -> Self {
        NonZeroU128::from(id).into()
    }
}

impl TryFrom<u32> for ID {
    type Error = SizeError;

    fn try_from(id: u32) -> Result<Self, Self::Error> {
        id.to_le_bytes().try_into()
    }
}

impl From<NonZeroU32> for ID {
    fn from(id: NonZeroU32) -> Self {
        NonZeroU128::from(id).into()
    }
}

impl TryFrom<u64> for ID {
    type Error = SizeError;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        id.to_le_bytes().try_into()
    }
}

impl From<NonZeroU64> for ID {
    fn from(id: NonZeroU64) -> Self {
        NonZeroU128::from(id).into()
    }
}

impl TryFrom<u128> for ID {
    type Error = SizeError;

    fn try_from(id: u128) -> Result<Self, Self::Error> {
        id.to_le_bytes().try_into()
    }
}

impl From<NonZeroU128> for ID {
    fn from(id: NonZeroU128) -> Self {
        Self(id.get().to_le_bytes())
    }
}

impl FromStr for ID {
    type Err = ParseIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseIDError {
                cause: "Empty strings are not valid".to_string(),
            });
        }

        if s.starts_with('0') {
            return Err(ParseIDError {
                cause: "Leading 0s are not valid".to_string(),
            });
        }

        let bs = u128::from_str_radix(s, 16).map_err(|e| ParseIDError {
            cause: e.to_string(),
        })?;
        ID::try_from(bs).map_err(|e| ParseIDError {
            cause: e.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ParseIDError {
    pub cause: String,
}

impl fmt::Debug for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = u128::from_le_bytes(self.0);
        let s = format!("{:02x}", id);
        let t = s.as_str().strip_prefix('0').unwrap_or(s.as_str());
        write!(f, "{}", t)
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

mod tests {
    #[test]
    fn parse_display() {
        let id = "1".parse::<crate::ID>().unwrap();
        assert_eq!(id.to_string(), "1");

        let id = "1bc0".parse::<crate::ID>().unwrap();
        assert_eq!(id.to_string(), "1bc0");

        let id = "ff00".parse::<crate::ID>().unwrap();
        assert_eq!(id.to_string(), "ff00");

        let id = "ff0".parse::<crate::ID>().unwrap();
        assert_eq!(id.to_string(), "ff0");

        let id = "abcd".parse::<crate::ID>().unwrap();
        assert_eq!(id.to_string(), "abcd");

        let id = "6bd9cb5f9f2644508fbbb0df1d6cce3a"
            .parse::<crate::ID>()
            .unwrap();
        assert_eq!(id.to_string(), "6bd9cb5f9f2644508fbbb0df1d6cce3a");

        "0".parse::<crate::ID>().unwrap_err();
        "0bcd".parse::<crate::ID>().unwrap_err();
        "6bd9cb5f9f2644508fbbb0df1d6cce3a0"
            .parse::<crate::ID>()
            .unwrap_err();
    }
}
