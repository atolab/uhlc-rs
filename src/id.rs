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
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU8;
use std::str::FromStr;
use uuid::Uuid;

/// An identifier for an HLC ([MAX_SIZE](ID::MAX_SIZE) bytes maximum).
/// This struct has a constant memory size (holding internally a `[u8; MAX_SIZE]` + a `NonZeroU8`),
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
/// let id = ID::try_from(&buf[..3]).unwrap();
/// assert_eq!(id.size(), 3);
/// assert_eq!(id.as_slice(), &[0x1a, 0x2b, 0x3c]);
/// assert_eq!(id.to_string(), "1A2B3C".to_string());
/// ```
///
/// ```
/// use uhlc::ID;
/// use uuid::Uuid;
///
/// let id = ID::from(Uuid::new_v4());
/// assert_eq!(id.size(), 16);
/// ```
#[derive(Copy, Clone, Eq, Deserialize, Serialize)]
pub struct ID {
    size: NonZeroU8,
    id: [u8; ID::MAX_SIZE],
}

impl ID {
    /// The maximum size of an ID in bytes: 16.
    pub const MAX_SIZE: usize = 16;

    /// The size of this ID in bytes
    #[inline]
    pub fn size(&self) -> usize {
        self.size.get() as _
    }

    /// This ID as a slice
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.id[..self.size()]
    }
}

impl From<Uuid> for ID {
    #[inline]
    fn from(uuid: Uuid) -> Self {
        uuid.as_bytes().try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SizeError(usize);
impl std::fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Maximum ID size ({} bytes) exceeded: {}",
            ID::MAX_SIZE,
            self.0
        )
    }
}
impl std::error::Error for SizeError {}
impl From<[u8; 16]> for ID {
    fn from(id: [u8; 16]) -> Self {
        Self {
            size: unsafe { NonZeroU8::new_unchecked(16) },
            id,
        }
    }
}
impl From<&[u8; 16]> for ID {
    fn from(id: &[u8; 16]) -> Self {
        (*id).into()
    }
}
macro_rules! impl_from_sized_slice_for_id {
    ($N: expr) => {
        impl From<&[u8; $N]> for ID {
            fn from(value: &[u8; $N]) -> Self {
                let mut id = std::mem::MaybeUninit::<[u8; 16]>::uninit();
                unsafe {
                    id.assume_init_mut()[..$N].copy_from_slice(value);
                    Self {
                        size: NonZeroU8::new_unchecked($N),
                        id: id.assume_init(),
                    }
                }
            }
        }
        impl From<[u8; $N]> for ID {
            fn from(id: [u8; $N]) -> Self {
                (&id).into()
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

impl TryFrom<&[u8]> for ID {
    type Error = SizeError;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let size = slice.len();
        if size > Self::MAX_SIZE {
            return Err(SizeError(size));
        }
        match NonZeroU8::new(size as u8) {
            Some(nz_size) => {
                let mut id = [0; 16];
                id[..size].copy_from_slice(slice);
                Ok(Self { size: nz_size, id })
            }
            None => Err(SizeError(size)),
        }
    }
}

impl PartialEq for ID {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.as_slice() == other.as_slice()
    }
}

impl PartialOrd for ID {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ID {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Hash for ID {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl fmt::Debug for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode_upper(self.as_slice()))
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for ID {
    type Err = ParseIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s)
            .map_err(|e| ParseIDError {
                cause: e.to_string(),
            })
            .and_then(|bytes| {
                ID::try_from(bytes.as_slice()).map_err(|e| ParseIDError {
                    cause: e.to_string(),
                })
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIDError {
    pub cause: String,
}
