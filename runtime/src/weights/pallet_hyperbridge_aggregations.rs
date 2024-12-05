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

//! Autogenerated weights for `pallet_hyperbridge_aggregations`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Rodrigos-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /Users/rdoria/horizen/new-zk-verify/zkVerify/target/production/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-hyperbridge-aggregations
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /Users/rdoria/horizen/new-zk-verify/zkVerify/HEADER-APACHE2
// --output
// /Users/rdoria/horizen/new-zk-verify/zkVerify/runtime/src/weights/pallet_hyperbridge_aggregations.rs
// --template
// /Users/rdoria/horizen/new-zk-verify/zkVerify/node/zkv-deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_hyperbridge_aggregations` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_hyperbridge_aggregations::WeightInfo for ZKVWeight<T> {
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Ismp::Nonce` (r:1 w:1)
    /// Proof: `Ismp::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: UNKNOWN KEY `0x52657175657374436f6d6d69746d656e74738d735dd361725e435d6df809b249` (r:1 w:1)
    /// Proof: UNKNOWN KEY `0x52657175657374436f6d6d69746d656e74738d735dd361725e435d6df809b249` (r:1 w:1)
    fn dispatch_aggregation() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `15`
        //  Estimated: `3593`
        // Minimum execution time: 36_000_000 picoseconds.
        Weight::from_parts(38_000_000, 3593)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
}
