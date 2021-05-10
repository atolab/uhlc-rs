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
use std::fmt;
use std::str::FromStr;

/// A timestamp made of a [`NTP64`] and a [`crate::HLC`]'s unique identifier.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
pub struct ParseTimestampError {
    pub cause: String,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_timestamp() {
        let id1: ID = ID::try_from(vec![0x01].as_ref()).unwrap();
        let id2: ID = ID::try_from(vec![0x02].as_ref()).unwrap();

        let ts1_epoch = Timestamp::new(Default::default(), id1.clone());
        assert_eq!(ts1_epoch.get_time().to_system_time(), UNIX_EPOCH);
        assert_eq!(ts1_epoch.get_id(), &id1);

        let ts2_epoch = Timestamp::new(Default::default(), id2.clone());
        assert_eq!(ts2_epoch.get_time().to_system_time(), UNIX_EPOCH);
        assert_eq!(ts2_epoch.get_id(), &id2);

        // Test that 2 Timestamps with same time but different ids are different and ordered
        assert_ne!(ts1_epoch, ts2_epoch);
        assert!(ts1_epoch < ts2_epoch);

        let now = system_time_clock();
        let ts1_now = Timestamp::new(now, id1);
        let ts2_now = Timestamp::new(now, id2);
        assert_ne!(ts1_now, ts2_now);
        assert!(ts1_now < ts2_now);
        assert!(ts1_epoch < ts1_now);
        assert!(ts2_epoch < ts2_now);

        let s = ts1_now.to_string();
        assert_eq!(ts1_now, s.parse().unwrap());
    }
}
