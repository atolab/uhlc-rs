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
use std::convert::TryFrom;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use uuid::Uuid;

/// An identifier for an HLC ([MAX_SIZE](ID::MAX_SIZE) bytes maximum).
/// This struct has a constant memory size (holding internally a `[u8; MAX_SIZE]` + a `usize`),
/// allowing allocations on the stack for better performances.
///
/// # Examples
///
/// ```
/// use uhlc::ID;
///
/// let buf = [0x1a, 0x2b, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00,
///            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
/// let id = ID::new(3, buf);
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
    size: usize,
    id: [u8; ID::MAX_SIZE],
}

impl ID {
    /// The maximum size of an ID in bytes: 16.
    pub const MAX_SIZE: usize = 16;

    /// Create a new ID with the "`size`" first bytes of "`id`"
    pub fn new(size: usize, id: [u8; ID::MAX_SIZE]) -> ID {
        ID { size, id }
    }

    /// The size of this ID in bytes
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// This ID as a slice
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.id[..self.size]
    }
}

impl From<Uuid> for ID {
    #[inline]
    fn from(uuid: Uuid) -> Self {
        ID {
            size: 16,
            id: *uuid.as_bytes(),
        }
    }
}

impl TryFrom<&[u8]> for ID {
    type Error = String;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let size = slice.len();
        if size > ID::MAX_SIZE {
            Err(format!(
                "Maximum ID size ({} bytes) exceeded: {}",
                ID::MAX_SIZE,
                size
            ))
        } else {
            let mut id = [0u8; ID::MAX_SIZE];
            id[..size].copy_from_slice(slice);
            Ok(ID::new(size, id))
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
            .and_then(|bytes| ID::try_from(bytes.as_slice()).map_err(|e| ParseIDError { cause: e }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIDError {
    pub cause: String,
}
