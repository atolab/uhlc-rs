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
use core::{fmt, time::Duration};
use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use core::str::FromStr;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// A timestamp made of a [`NTP64`] and a [`crate::HLC`]'s unique identifier.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
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
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.time, self.id)
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}/{:?}", self.time, self.id)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Timestamp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}/{}", self.time, self.id);
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
#[cfg_attr(deature = "defmt", derive(defmt::Format))]
pub struct ParseTimestampError {
    pub cause: String,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;

    #[test]
    fn test_timestamp() {
        let id1: ID = ID::try_from([0x01]).unwrap();
        let id2: ID = ID::try_from([0x02]).unwrap();

        let ts1_epoch = Timestamp::new(Default::default(), id1);
        #[cfg(feature = "std")]
        assert_eq!(ts1_epoch.get_time().to_system_time(), std::time::UNIX_EPOCH);
        assert_eq!(ts1_epoch.get_id(), &id1);

        let ts2_epoch = Timestamp::new(Default::default(), id2);
        #[cfg(feature = "std")]
        assert_eq!(ts2_epoch.get_time().to_system_time(), std::time::UNIX_EPOCH);
        assert_eq!(ts2_epoch.get_id(), &id2);

        // Test that 2 Timestamps with same time but different ids are different and ordered
        assert_ne!(ts1_epoch, ts2_epoch);
        assert!(ts1_epoch < ts2_epoch);

        #[cfg(feature = "std")]
        {
            let now = system_time_clock();
            let ts1_now = Timestamp::new(now, id1);
            let ts2_now = Timestamp::new(now, id2);
            assert_ne!(ts1_now, ts2_now);
            assert!(ts1_now < ts2_now);
            assert!(ts1_epoch < ts1_now);
            assert!(ts2_epoch < ts2_now);

            let s = ts1_now.to_string();
            assert_eq!(ts1_now, s.parse().unwrap());

            let diff = ts1_now.get_diff_duration(&ts2_now);
            assert_eq!(diff, Duration::from_secs(0));
        }
    }
}
