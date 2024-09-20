// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `crate::parachains::inclusion`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-09-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// crate::parachains::inclusion
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// HEADER-APACHE2
// --output
// runtime/src/weights/parachains/inclusion.rs
// --template
// node/zkv-deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `crate::parachains::inclusion` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> crate::parachains::inclusion::WeightInfo for ZKVWeight<T> {
    /// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
    /// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
    /// Storage: `MessageQueue::Pages` (r:1 w:999)
    /// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(32818), added: 35293, mode: `MaxEncodedLen`)
    /// Storage: UNKNOWN KEY `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
    /// Proof: UNKNOWN KEY `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
    /// Storage: UNKNOWN KEY `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
    /// Proof: UNKNOWN KEY `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
    /// The range of component `i` is `[1, 1000]`.
    fn receive_upward_messages(i: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `32993`
        //  Estimated: `36283`
        // Minimum execution time: 59_920_000 picoseconds.
        Weight::from_parts(60_726_000, 36283)
            // Standard Error: 233_369
            .saturating_add(Weight::from_parts(48_715_884, 0).saturating_mul(i.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
    }
}
