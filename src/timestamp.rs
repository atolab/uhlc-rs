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
use super::{ID, NTP64};
use alloc::string::String;
use core::{fmt, time::Duration};
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use core::str::FromStr;

/// A timestamp made of a [`NTP64`] and a [`crate::HLC`]'s unique identifier.
///
/// ## Conversion to/from String
/// A Timestamp is formatted to a String as such:  `"<ntp64_time>/<hlc_id_hexadecimal>"`
/// 2 different String representations are supported:
/// 1. **`<ntp64_time>` as an unsigned integer in decimal format**
///   - Such conversion is lossless and thus bijective.
///   - Timestamp to String: use [`std::fmt::Display::fmt()`] or [`std::string::ToString::to_string()`].
///   - String to Timestamp: use [`std::str::FromStr::from_str()`]
/// 2. **`<ntp64_time>`as a [RFC3339](https://www.rfc-editor.org/rfc/rfc3339.html#section-5.8) (human readable) format**:
///   - Such conversion loses some precision because of rounding when conferting the fraction part to nanoseconds
///   - As a consequence it's not bijective: a Timestamp converted to RFC3339 String and then converted back to Timestamp might result to a different time.
///   - Timestamp to String: use [`std::fmt::Display::fmt()`] with the alternate flag (`{:#}`) or [`Timestamp::to_string_rfc3339_lossy()`].
///   - String to Timestamp: use [`Timestamp::parse_rfc3339()`]
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Timestamp {
    time: NTP64,
    id: ID,
}

impl Timestamp {
    // Create a [`Timestamp`] with a [`NTP64`] and a [`crate::HLC`]'s unique `id`.
    #[inline]
    pub fn new(time: NTP64, id: ID) -> Timestamp {
        Timestamp { time, id }
    }

    // Returns the [`NTP64`] time.
    #[inline]
    pub fn get_time(&self) -> &NTP64 {
        &self.time
    }

    // Returns the [`crate::HLC`]'s unique `id`.
    #[inline]
    pub fn get_id(&self) -> &ID {
        &self.id
    }

    // Returns the time difference between two timestamps as a [`Duration`].
    #[inline]
    pub fn get_diff_duration(&self, other: &Timestamp) -> Duration {
        (self.time - other.time).to_duration()
    }

    /// Convert to a RFC3339 time representation with nanoseconds precision.
    /// e.g.: `"2024-07-01T13:51:12.129693000Z/33"``
    #[cfg(feature = "std")]
    pub fn to_string_rfc3339_lossy(&self) -> String {
        #[cfg(feature = "std")]
        return format!("{:#}", self);
        #[cfg(not(feature = "std"))]
        return self.to_string();
    }

    /// Parse a RFC3339 time representation into a NTP64.
    #[cfg(feature = "std")]
    pub fn parse_rfc3339(s: &str) -> Result<Self, ParseTimestampError> {
        match s.find('/') {
            Some(i) => {
                let (stime, srem) = s.split_at(i);
                let time = NTP64::parse_rfc3339(stime)
                    .map_err(|e| ParseTimestampError { cause: e.cause })?;
                let id =
                    ID::from_str(&srem[1..]).map_err(|e| ParseTimestampError { cause: e.cause })?;
                Ok(Timestamp::new(time, id))
            }
            None => Err(ParseTimestampError {
                cause: "No '/' found in String".into(),
            }),
        }
    }
}

impl fmt::Display for Timestamp {
    /// Formats Timestamp as the time part followed by the ID part, with `/` as separator.  
    /// By default the time part is formatted as an unsigned integer in decimal format.  
    /// If the alternate flag `{:#}` is used, the time part is formatted with RFC3339 representation with nanoseconds precision.
    ///
    /// # Examples
    /// ```
    ///   use uhlc::*;
    ///   use std::convert::TryFrom;
    ///
    ///   let t =Timestamp::new(NTP64(7386690599959157260), ID::try_from([0x33]).unwrap());
    ///   println!("{t}");    // displays: 7386690599959157260/33
    ///   println!("{t:#}");  // displays: 2024-07-01T15:32:06.860479000Z/33
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#}/{}", self.time, self.id)
        } else {
            write!(f, "{}/{}", self.time, self.id)
        }
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}/{:?}", self.time, self.id)
    }
}

#[cfg(feature = "std")]
impl FromStr for Timestamp {
    type Err = ParseTimestampError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('/') {
            Some(i) => {
                let (stime, srem) = s.split_at(i);
                let time =
                    NTP64::from_str(stime).map_err(|e| ParseTimestampError { cause: e.cause })?;
                let id =
                    ID::from_str(&srem[1..]).map_err(|e| ParseTimestampError { cause: e.cause })?;
                Ok(Timestamp::new(time, id))
            }
            None => Err(ParseTimestampError {
                cause: "No '/' found in String".into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ParseTimestampError {
    pub cause: String,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use core::convert::TryFrom;

    #[test]
    fn test_timestamp() {
        let id1: ID = ID::try_from([0x01]).unwrap();
        let id2: ID = ID::try_from([0x02]).unwrap();

        let ts1_epoch = Timestamp::new(Default::default(), id1);
        #[cfg(feature = "std")]
        assert_eq!(ts1_epoch.get_time().to_system_time(), std::time::UNIX_EPOCH);
        #[cfg(not(feature = "std"))]
        assert_eq!(ts1_epoch.get_time().as_u64(), 0);
        assert_eq!(ts1_epoch.get_id(), &id1);

        let ts2_epoch = Timestamp::new(Default::default(), id2);
        #[cfg(feature = "std")]
        assert_eq!(ts2_epoch.get_time().to_system_time(), std::time::UNIX_EPOCH);
        #[cfg(not(feature = "std"))]
        assert_eq!(ts2_epoch.get_time().as_u64(), 0);
        assert_eq!(ts2_epoch.get_id(), &id2);

        // Test that 2 Timestamps with same time but different ids are different and ordered
        assert_ne!(ts1_epoch, ts2_epoch);
        assert!(ts1_epoch < ts2_epoch);

        #[cfg(feature = "std")]
        let now = system_time_clock();
        #[cfg(not(feature = "std"))]
        let now = zero_clock();
        let ts1_now = Timestamp::new(now, id1);
        let ts2_now = Timestamp::new(now, id2);
        assert_ne!(ts1_now, ts2_now);
        assert!(ts1_now < ts2_now);

        #[cfg(feature = "std")]
        {
            // These are not necessarily true in no_std since we use a new zero-based (incremental) clock
            assert!(ts1_epoch < ts1_now);
            assert!(ts2_epoch < ts2_now);
        }

        #[cfg(feature = "std")]
        {
            // We do not care about parsing human-readable timestamps in no_std
            let s = ts1_now.to_string();
            assert_eq!(ts1_now, s.parse().unwrap());
        }

        let diff = ts1_now.get_diff_duration(&ts2_now);
        assert_eq!(diff, Duration::from_secs(0));
    }

    #[test]
    fn bijective_to_string() {
        use crate::*;
        use std::str::FromStr;

        let hlc = HLCBuilder::new().with_id(ID::rand()).build();
        for _ in 1..10000 {
            let now_ts = hlc.new_timestamp();
            assert_eq!(now_ts, Timestamp::from_str(&now_ts.to_string()).unwrap());
        }
    }
}
