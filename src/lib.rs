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

//! A Unique Hybrid Logical Clock.
//!
//! This library is an implementation of an
//! [Hybrid Logical Clock (HLC)](https://cse.buffalo.edu/tech-reports/2014-04.pdf)
//! associated to a unique identifier.
//! Thus, it is able to generate timestamps that are unique across a distributed system,
//! without the need of a centralized time source.
//!
//! # Quick Start
//!
//! ```
//! use uhlc::HLC;
//!
//! // create an HLC with a generated random ID and relying on SystemTime::now()
//! let hlc = HLC::default();
//!
//! // generate timestamps
//! let ts1 = hlc.new_timestamp();
//! let ts2 = hlc.new_timestamp();
//! assert!(ts2 > ts1);
//!
//! // update the HLC with a timestamp incoming from another HLC
//! // (typically remote, but not in this example...)
//! let hlc2 = HLC::default();
//! let other_ts = hlc2.new_timestamp();
//!
//! if ! hlc.update_with_timestamp(&other_ts).is_ok() {
//!     println!(r#"The incoming timestamp would make this HLC
//!              to drift too much. You should refuse it!"#);
//! }
//!
//! let ts3 = hlc.new_timestamp();
//! assert!(ts3 > ts2);
//! assert!(ts3 > other_ts);
//! ```

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://atolab.github.io/uhlc-rs/"
)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::{format, string::String};
use core::cmp;
use core::time::Duration;

#[cfg(feature = "std")]
use {
    lazy_static::lazy_static,
    std::env::var,
    std::sync::Mutex,
    std::time::{SystemTime, UNIX_EPOCH},
};

#[cfg(not(feature = "std"))]
use spin::Mutex; // No_std-friendly alternative to std::sync::Mutex

mod id;
pub use id::*;

mod ntp64;
pub use ntp64::*;

mod timestamp;
pub use timestamp::*;

/// The size of counter part in [`NTP64`] (in bits)
pub const CSIZE: u8 = 4u8;
// Bit-mask of the counter part within the 64 bits time
const CMASK: u64 = (1u64 << CSIZE) - 1u64;
// Bit-mask of the logical clock part within the 64 bits time
const LMASK: u64 = !CMASK;

// HLC Delta in milliseconds: maximum accepted drift for an external timestamp.
// I.e.: if an incoming timestamp has a time > now() + delta, then the HLC is not updated.
const DEFAULT_DELTA_MS: u64 = 500;
#[cfg(feature = "std")]
lazy_static! {
    static ref DELTA_MS: u64 = match var("UHLC_MAX_DELTA_MS") {
        Ok(s) => s.parse().unwrap_or_else(|e| panic!(
            "Error parsing environment variable ${{UHLC_MAX_DELTA_MS}}={} : {}",
            s, e
        )),
        Err(std::env::VarError::NotPresent) => DEFAULT_DELTA_MS,
        Err(e) => panic!(
            "Error parsing environment variable ${{UHLC_MAX_DELTA_MS}}: {}",
            e
        ),
    };
}
#[cfg(not(feature = "std"))]
static DELTA_MS: &u64 = &DEFAULT_DELTA_MS; // Environment variables do not make sense in no_std environment

///
/// The builder of [`HLC`].
///
/// # Examples
///
/// ```
/// use std::{convert::TryFrom, time::Duration};
/// use uhlc::{HLCBuilder, ID};
///
/// let default_hlc = HLCBuilder::new().build();
/// println!("{}", default_hlc.new_timestamp());
///
/// let custom_hlc = HLCBuilder::new()
///    .with_id(ID::try_from([0x01, 0x02, 0x03]).unwrap())
///    .with_max_delta(Duration::from_secs(1))
///    .build();
/// println!("{}", custom_hlc.new_timestamp());
pub struct HLCBuilder {
    hlc: HLC,
}

impl HLCBuilder {
    ///
    /// Constructs a new HLCBuilder for the creation of an [`HLC`], with the following default configuration:
    ///  * a random u128 as HLC identifier.
    ///    Can be changed calling [`Self::with_id()`].
    ///  * [`system_time_clock()`] as physical clock (i.e. the ).
    ///    Can be changed calling [`Self::with_clock()`].
    ///  * 500 millisecond as maximum delta (i.e. the maximum accepted drift for an external timestamp).
    ///    Can be changed calling [`Self::with_max_delta()`].
    ///
    pub fn new() -> HLCBuilder {
        HLCBuilder::default()
    }

    ///
    /// Configure a specific identifier for the HLC to be created.
    ///
    /// **NOTE: the identifier must be unique in the system.**
    ///
    pub fn with_id(mut self, id: ID) -> HLCBuilder {
        self.hlc.id = id;
        self
    }

    ///
    /// Configure a specific physical clock for the HLC to be created.
    ///
    /// The `clock` parameter must be a function returning a new physical time (as an [`NTP64`] at each call.
    /// The time returned by this clock doesn't need to be monotonic: when the HLC generates a new timestamp from this time,
    /// it first checks if this time is greater than the previously generated timestamp. If not, the new timestamp it the previous one +1.
    ///
    pub fn with_clock(mut self, clock: fn() -> NTP64) -> HLCBuilder {
        self.hlc.clock = clock;
        self
    }

    ///
    /// Configure the maximum delta accepted by an HLC when updating it's logical clock calling [`HLC::update_with_timestamp()`].
    ///
    pub fn with_max_delta(mut self, delta: Duration) -> HLCBuilder {
        self.hlc.delta = delta.into();
        self
    }

    pub fn build(self) -> HLC {
        self.hlc
    }
}

impl Default for HLCBuilder {
    fn default() -> Self {
        HLCBuilder {
            hlc: HLC {
                id: ID::rand(),
                #[cfg(feature = "std")]
                clock: system_time_clock,
                #[cfg(not(feature = "std"))]
                clock: zero_clock,
                delta: NTP64::from(Duration::from_millis(*DELTA_MS)),
                last_time: Default::default(),
            },
        }
    }
}

/// An Hybric Logical Clock generating [`Timestamp`]s
pub struct HLC {
    id: ID,
    clock: fn() -> NTP64,
    delta: NTP64,
    last_time: Mutex<NTP64>,
}

#[cfg(feature = "std")]
macro_rules! lock {
    ($var:expr) => {
        match $var.try_lock() {
            Ok(guard) => guard,
            Err(_) => $var.lock().unwrap(),
        }
    };
}

#[cfg(not(feature = "std"))]
macro_rules! lock {
    ($var:expr) => {
        $var.lock()
    };
}

impl HLC {
    /// Generate a new [`Timestamp`].
    ///
    /// This timestamp is unique in the system and is always greater
    /// than the latest timestamp generated by the HLC and than the
    /// latest incoming timestamp that was used to update this [`HLC`]
    /// (using [`HLC::update_with_timestamp()`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use uhlc::HLC;
    ///
    /// let hlc = HLC::default();
    /// let ts1 =  hlc.new_timestamp();
    /// let ts2 =  hlc.new_timestamp();
    /// assert!(ts2 > ts1);
    /// ```
    pub fn new_timestamp(&self) -> Timestamp {
        let mut now = (self.clock)();
        now.0 &= LMASK;
        let mut last_time = lock!(self.last_time);
        if now.0 > (last_time.0 & LMASK) {
            *last_time = now
        } else {
            *last_time += 1;
        }
        Timestamp::new(*last_time, self.id)
    }

    /// Returns the HLC [`ID`].
    ///
    /// This ID is the specific identifier for this HLC instance.
    ///
    pub fn get_id(&self) -> &ID {
        &self.id
    }

    /// Returns the HLC delta as [`NTP64`].
    ///
    /// The maximum delta accepted by an HLC when updating it's logical clock calling [`HLC::update_with_timestamp()`].
    ///
    pub fn get_delta(&self) -> &NTP64 {
        &self.delta
    }

    /// Update this [`HLC`] with a [`Timestamp`].
    ///
    /// Typically, this timestamp should have been generated by another HLC.
    /// If the timestamp exceeds the current time of this HLC by more than the configured maximum delta
    /// (see [`HLCBuilder::with_max_delta()`]) an [`Err`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use uhlc::HLC;
    ///
    /// let hlc1 = HLC::default();
    ///
    /// // update the HLC with a timestamp incoming from another HLC
    /// // (typically remote, but not in this example...)
    /// let hlc2 = HLC::default();
    /// let other_ts = hlc2.new_timestamp();
    /// if ! hlc1.update_with_timestamp(&other_ts).is_ok() {
    ///     println!(r#"The incoming timestamp would make this HLC
    ///              to drift too much. You should refuse it!"#);
    /// }
    ///
    /// let ts = hlc1.new_timestamp();
    /// assert!(ts > other_ts);
    /// ```
    pub fn update_with_timestamp(&self, timestamp: &Timestamp) -> Result<(), String> {
        let mut now = (self.clock)();
        now.0 &= LMASK;
        let msg_time = timestamp.get_time();
        if *msg_time > now && *msg_time - now > self.delta {
            let err_msg = format!(
                "incoming timestamp from {} exceeding delta {}ms is rejected: {:#} vs. now: {:#}",
                timestamp.get_id(),
                self.delta.to_duration().as_millis(),
                msg_time,
                now
            );
            #[cfg(feature = "std")]
            log::warn!("{}", err_msg);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", err_msg);
            Err(err_msg)
        } else {
            let mut last_time = lock!(self.last_time);
            let max_time = cmp::max(cmp::max(now, *msg_time), *last_time);
            if max_time == now {
                *last_time = now;
            } else if max_time == *msg_time {
                *last_time = *msg_time + 1;
            } else {
                *last_time += 1;
            }
            Ok(())
        }
    }
}

impl Default for HLC {
    /// Create a new [`HLC`] with a random u128 ID and using
    /// [`system_time_clock()`] as physical clock.
    /// This is equivalent to `HLCBuilder::default().build()`
    fn default() -> Self {
        HLCBuilder::default().build()
    }
}

/// A physical clock relying on std::time::SystemTime::now().
///
/// It returns a NTP64 relative to std::time::UNIX_EPOCH (1st Jan 1970).
/// That's the default clock used by an [`HLC`] if [`HLCBuilder::with_clock()`] is not called.
///
#[inline]
#[cfg(feature = "std")]
pub fn system_time_clock() -> NTP64 {
    NTP64::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap())
}

/// A physical clock relying on CLOCK_MONOTONIC.
///
/// Note: The time produced by this clock is relative to an unspecified starting point (likely the host boot).
/// Therefore, comparing and ordering times produced by different instances of this clock on different hosts would probably be meaningless.
///
/// See: https://linux.die.net/man/2/clock_gettime
///
#[inline]
#[cfg(all(feature = "nix", target_family = "unix"))]
pub fn monotonic_time_clock() -> NTP64 {
    let now = nix::time::ClockId::CLOCK_MONOTONIC.now().unwrap();
    NTP64::from(now)
}

/// A dummy clock that returns a NTP64 initialized with the value 0.
/// Suitable to use in no_std environments where std::time::{SystemTime, UNIX_EPOCH} are not available.
/// If the feature `std` is disabled, that's the default clock used by an [`HLC`] if [`HLCBuilder::with_clock()`] is not called.
/// Notice that this means that the [`HLC`] will use incremental timestamps starting from 0.
#[inline]
pub fn zero_clock() -> NTP64 {
    NTP64(0)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use async_std::sync::Arc;
    use async_std::task;
    use core::convert::TryFrom;
    use core::time::Duration;
    use futures::join;

    fn is_sorted(vec: &[Timestamp]) -> bool {
        let mut it = vec.iter();
        let mut ts = it.next().unwrap();
        for next in it {
            if next <= ts {
                return false;
            };
            ts = next;
        }
        true
    }

    #[test]
    fn hlc_parallel() {
        use alloc::vec::Vec;
        task::block_on(async {
            let id0: ID = ID::try_from([0x01]).unwrap();
            let id1: ID = ID::try_from([0x02]).unwrap();
            let id2: ID = ID::try_from([0x03]).unwrap();
            let id3: ID = ID::try_from([0x04]).unwrap();
            let hlc0 = Arc::new(HLCBuilder::new().with_id(id0).build());
            let hlc1 = Arc::new(HLCBuilder::new().with_id(id1).build());
            let hlc2 = Arc::new(HLCBuilder::new().with_id(id2).build());
            let hlc3 = Arc::new(HLCBuilder::new().with_id(id3).build());

            // Make 4 tasks to generate 10000 timestamps each with distinct HLCs,
            // and also to update each other HLCs
            const NB_TIME: usize = 10000;
            let t0 = {
                let hlc0 = hlc0.clone();
                let hlc1 = hlc1.clone();
                task::spawn(async move {
                    let mut times: Vec<Timestamp> = Vec::with_capacity(10000);
                    for _ in 0..NB_TIME {
                        let ts = hlc0.new_timestamp();
                        assert!(hlc1.update_with_timestamp(&ts).is_ok());
                        times.push(ts)
                    }
                    times
                })
            };
            let t1 = {
                let hlc1 = hlc1.clone();
                let hlc2 = hlc2.clone();
                task::spawn(async move {
                    let mut times: Vec<Timestamp> = Vec::with_capacity(10000);
                    for _ in 0..NB_TIME {
                        let ts = hlc1.new_timestamp();
                        assert!(hlc2.update_with_timestamp(&ts).is_ok());
                        times.push(ts)
                    }
                    times
                })
            };
            let t2 = {
                let hlc2 = hlc3.clone();
                let hlc3 = hlc3.clone();
                task::spawn(async move {
                    let mut times: Vec<Timestamp> = Vec::with_capacity(10000);
                    for _ in 0..NB_TIME {
                        let ts = hlc2.new_timestamp();
                        assert!(hlc3.update_with_timestamp(&ts).is_ok());
                        times.push(ts)
                    }
                    times
                })
            };
            let t3 = {
                let hlc3 = hlc3.clone();
                let hlc0 = hlc0.clone();
                task::spawn(async move {
                    let mut times: Vec<Timestamp> = Vec::with_capacity(10000);
                    for _ in 0..NB_TIME {
                        let ts = hlc3.new_timestamp();
                        assert!(hlc0.update_with_timestamp(&ts).is_ok());
                        times.push(ts)
                    }
                    times
                })
            };
            let vecs = join!(t0, t1, t2, t3);

            // test that each timeseries is sorted (i.e. monotonic time)
            assert!(is_sorted(&vecs.0));
            assert!(is_sorted(&vecs.1));
            assert!(is_sorted(&vecs.2));
            assert!(is_sorted(&vecs.3));

            // test that there is no duplicate amongst all timestamps
            let mut all_times: Vec<Timestamp> = vecs
                .0
                .into_iter()
                .chain(vecs.1)
                .chain(vecs.2)
                .chain(vecs.3)
                .collect::<Vec<Timestamp>>();
            assert_eq!(NB_TIME * 4, all_times.len());
            all_times.sort();
            all_times.dedup();
            assert_eq!(NB_TIME * 4, all_times.len());
        });
    }

    #[test]
    fn hlc_update_with_timestamp() {
        let id: ID = ID::rand();
        let hlc = HLCBuilder::new().with_id(id).build();

        // Test that updating with an old Timestamp don't break the HLC
        let past_ts = Timestamp::new(Default::default(), id);
        let now_ts = hlc.new_timestamp();
        assert!(hlc.update_with_timestamp(&past_ts).is_ok());
        assert!(hlc.new_timestamp() > now_ts);

        // Test that updating with a Timestamp exceeding the delta is refused
        let now_ts = hlc.new_timestamp();
        let future_time = now_ts.get_time() + NTP64::from(Duration::from_millis(1000));
        let future_ts = Timestamp::new(future_time, id);
        assert!(hlc.update_with_timestamp(&future_ts).is_err())
    }

    #[cfg(all(feature = "nix", target_family = "unix"))]
    #[test]
    fn hlc_nix_monotonic() {
        let hlc = HLCBuilder::new().with_clock(monotonic_time_clock).build();
        let t1 = monotonic_time_clock();
        // Sleep for (CMASK + 1) nanoseconds since HLC clock strips off the lower CMASK bits.
        std::thread::sleep(Duration::from_nanos(CMASK + 1));
        let t2 = hlc.new_timestamp();
        std::thread::sleep(Duration::from_nanos(1));
        let t3 = monotonic_time_clock();

        assert!(t2.get_time() > &t1);
        assert!(&t3 > t2.get_time());
    }
}
