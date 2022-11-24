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
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration;
use core::fmt;

#[cfg(feature = "std")]
use {
    humantime::{format_rfc3339_nanos, parse_rfc3339},
    std::time::{SystemTime, UNIX_EPOCH},
    core::str::FromStr,
};

#[cfg(not(feature = "std"))]
use alloc::string::String;

// maximal number of seconds that can be represented in the 32-bits part
const MAX_NB_SEC: u64 = (1u64 << 32) - 1;
// number of NTP fraction per second (2^32)
const FRAC_PER_SEC: u64 = 1u64 << 32;
// Bit-mask for the fraction of a second part within an NTP timestamp
const FRAC_MASK: u64 = 0xFFFF_FFFFu64;

// number of nanoseconds in 1 second
const NANO_PER_SEC: u64 = 1_000_000_000;

/// A NTP 64-bits format as specified in
/// [RFC-5909](https://tools.ietf.org/html/rfc5905#section-6)
///
/// The first 32-bits part is the number of second since the EPOCH of the physical clock,
/// and the second 32-bits part is the fraction of second.  
/// In case it's part of a [`crate::Timestamp`] generated by an [`crate::HLC`] the last few bits are replaced
/// by the HLC logical counter. The size of this counter currently hard-coded in [`crate::CSIZE`].
///
/// Note that this timestamp in actually similar to a [`std::time::Duration`], as it doesn't
/// define an EPOCH. Only the [`NTP64::to_system_time()`] and [`std::fmt::Display::fmt()`] operations assume that
/// it's relative to UNIX_EPOCH (1st Jan 1970) to display the timpestamp in RFC-3339 format.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize)]
pub struct NTP64(pub u64);

impl NTP64 {
    /// Returns this NTP64 as a u64.
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Returns the 32-bits seconds part.
    #[inline]
    pub fn as_secs(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// Returns the 32-bits fraction of second part converted to nanoseconds.
    #[inline]
    pub fn subsec_nanos(&self) -> u32 {
        let frac = self.0 & FRAC_MASK;
        ((frac * NANO_PER_SEC) / FRAC_PER_SEC) as u32
    }

    /// Convert to a [`Duration`].
    #[inline]
    pub fn to_duration(self) -> Duration {
        Duration::new(self.as_secs().into(), self.subsec_nanos())
    }

    /// Convert to a [`SystemTime`] (making the assumption that this NTP64 is relative to [`UNIX_EPOCH`]).
    #[inline]
    #[cfg(feature = "std")]
    pub fn to_system_time(self) -> SystemTime {
        UNIX_EPOCH + self.to_duration()
    }
}

impl Add for NTP64 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl<'a> Add<NTP64> for &'a NTP64 {
    type Output = <NTP64 as Add<NTP64>>::Output;

    #[inline]
    fn add(self, other: NTP64) -> <NTP64 as Add<NTP64>>::Output {
        Add::add(*self, other)
    }
}

impl Add<&NTP64> for NTP64 {
    type Output = <NTP64 as Add<NTP64>>::Output;

    #[inline]
    fn add(self, other: &NTP64) -> <NTP64 as Add<NTP64>>::Output {
        Add::add(self, *other)
    }
}

impl Add<&NTP64> for &NTP64 {
    type Output = <NTP64 as Add<NTP64>>::Output;

    #[inline]
    fn add(self, other: &NTP64) -> <NTP64 as Add<NTP64>>::Output {
        Add::add(*self, *other)
    }
}

impl Add<u64> for NTP64 {
    type Output = Self;

    #[inline]
    fn add(self, other: u64) -> Self {
        Self(self.0 + other)
    }
}

impl AddAssign<u64> for NTP64 {
    #[inline]
    fn add_assign(&mut self, other: u64) {
        *self = Self(self.0 + other);
    }
}

impl Sub for NTP64 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl<'a> Sub<NTP64> for &'a NTP64 {
    type Output = <NTP64 as Sub<NTP64>>::Output;

    #[inline]
    fn sub(self, other: NTP64) -> <NTP64 as Sub<NTP64>>::Output {
        Sub::sub(*self, other)
    }
}

impl Sub<&NTP64> for NTP64 {
    type Output = <NTP64 as Sub<NTP64>>::Output;

    #[inline]
    fn sub(self, other: &NTP64) -> <NTP64 as Sub<NTP64>>::Output {
        Sub::sub(self, *other)
    }
}

impl Sub<&NTP64> for &NTP64 {
    type Output = <NTP64 as Sub<NTP64>>::Output;

    #[inline]
    fn sub(self, other: &NTP64) -> <NTP64 as Sub<NTP64>>::Output {
        Sub::sub(*self, *other)
    }
}

impl Sub<u64> for NTP64 {
    type Output = Self;

    #[inline]
    fn sub(self, other: u64) -> Self {
        Self(self.0 - other)
    }
}

impl SubAssign<u64> for NTP64 {
    #[inline]
    fn sub_assign(&mut self, other: u64) {
        *self = Self(self.0 - other);
    }
}

impl fmt::Display for NTP64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "std")]
        return write!(f, "{}", format_rfc3339_nanos(self.to_system_time()));
        #[cfg(not(feature = "std"))]
        return write!(f, "{:x}", self.0);
    }
}

impl fmt::Debug for NTP64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl From<Duration> for NTP64 {
    fn from(duration: Duration) -> NTP64 {
        let secs = duration.as_secs();
        assert!(secs <= MAX_NB_SEC);
        let nanos: u64 = duration.subsec_nanos().into();
        NTP64((secs << 32) + ((nanos * FRAC_PER_SEC) / NANO_PER_SEC) + 1)
    }
}

#[cfg(feature = "std")]
impl FromStr for NTP64 {
    type Err = ParseNTP64Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_rfc3339(s)
            .map_err(|e| ParseNTP64Error {
                cause: e.to_string(),
            })
            .and_then(|time| {
                time.duration_since(UNIX_EPOCH)
                    .map_err(|e| ParseNTP64Error {
                        cause: e.to_string(),
                    })
            })
            .map(NTP64::from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNTP64Error {
    pub cause: String,
}
