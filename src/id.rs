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
use alloc::string::{String, ToString};
use core::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    fmt,
    hash::Hash,
    num::NonZeroU128,
    str::FromStr,
};
use serde::{Deserialize, Serialize};
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
#[derive(Copy, Clone, Eq, Deserialize, Serialize, PartialEq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ID(NonZeroU128);

impl ID {
    /// The maximum size of an ID in bytes: 16.
    pub const MAX_SIZE: usize = 16;

    /// The size of this ID in bytes
    #[inline]
    pub fn size(&self) -> usize {
        // Safety: here, we're voluntarily bypassing the platform's endianness.
        // All constructors MUST ensure the value is actually LE encoded.
        Self::MAX_SIZE
            - (if cfg!(target_endian = "little") {
                self.0.leading_zeros()
            } else {
                self.0.trailing_zeros()
            } / 8) as usize
    }

    /// This ID as a slice
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        // Safety: here, we're voluntarily ignoring the platform's endianness.
        // All constructors MUST ensure the value is actually LE encoded.
        let slice = unsafe { core::mem::transmute::<&NonZeroU128, &[u8; 16]>(&self.0) };
        &slice[..self.size()]
    }
}

impl From<ID> for Uuid {
    #[inline]
    fn from(id: ID) -> Self {
        Uuid::from_u128(id.0.get())
    }
}

impl From<Uuid> for ID {
    #[inline]
    fn from(uuid: Uuid) -> Self {
        uuid.as_u128()
            .to_le_bytes()
            .try_into()
            .expect("Uuid should always be non-null")
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
            fn try_from(value: &[u8; $N]) -> Result<Self, Self::Error> {
                let mut id = 0u128;
                // Safety: all operations on the u128s constructed from slices treat them as little-endian.
                // Always constructing as little endian from a slice makes for less surprising behaviours when
                // inspecting on wire.
                unsafe {
                    core::mem::transmute::<&mut u128, &mut [u8; 16]>(&mut id)[..$N]
                        .copy_from_slice(value);
                }
                match NonZeroU128::new(id) {
                    Some(id) => Ok(Self(id)),
                    None => Err(SizeError(0)),
                }
            }
        }
        impl TryFrom<[u8; $N]> for ID {
            type Error = SizeError;
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
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let size = slice.len();
        if size > Self::MAX_SIZE {
            return Err(SizeError(size));
        }
        let mut id = 0u128;
        unsafe {
            core::mem::transmute::<&mut u128, &mut [u8; Self::MAX_SIZE]>(&mut id)[..size]
                .copy_from_slice(slice);
            match NonZeroU128::new(id) {
                Some(id) => Ok(Self(id)),
                None => Err(SizeError(0)),
            }
        }
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
        // Safety: here, we're voluntarily bypassing the platform's endianness.
        // All constructors MUST ensure the value is actually LE encoded.
        if cfg!(target_endian = "little") {
            self.0.cmp(&other.0)
        } else {
            u128::from_le(self.0.get()).cmp(&u128::from_le(other.0.get()))
        }
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ParseIDError {
    pub cause: String,
}
