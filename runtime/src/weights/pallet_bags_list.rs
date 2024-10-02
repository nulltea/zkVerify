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

//! Autogenerated weights for `pallet_bags_list`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-10-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `e7b272a34a72`, CPU: `AMD Ryzen 7 7700X 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-bags-list
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_bags_list.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.MuprxYSrGl

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_bags_list` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_bags_list::WeightInfo for ZKVWeight<T> {
    /// Storage: `Staking::Bonded` (r:1 w:0)
    /// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Staking::Ledger` (r:1 w:0)
    /// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(874), added: 3349, mode: `MaxEncodedLen`)
    /// Storage: `VoterList::ListNodes` (r:4 w:4)
    /// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
    /// Storage: `VoterList::ListBags` (r:1 w:1)
    /// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
    fn rebag_non_terminal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1565`
        //  Estimated: `11506`
        // Minimum execution time: 54_480_000 picoseconds.
        Weight::from_parts(55_600_000, 11506)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `Staking::Bonded` (r:1 w:0)
    /// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Staking::Ledger` (r:1 w:0)
    /// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(874), added: 3349, mode: `MaxEncodedLen`)
    /// Storage: `VoterList::ListNodes` (r:3 w:3)
    /// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
    /// Storage: `VoterList::ListBags` (r:2 w:2)
    /// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
    fn rebag_terminal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1459`
        //  Estimated: `8877`
        // Minimum execution time: 52_011_000 picoseconds.
        Weight::from_parts(53_310_000, 8877)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `VoterList::ListNodes` (r:4 w:4)
    /// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
    /// Storage: `Staking::Bonded` (r:2 w:0)
    /// Proof: `Staking::Bonded` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Staking::Ledger` (r:2 w:0)
    /// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(874), added: 3349, mode: `MaxEncodedLen`)
    /// Storage: `VoterList::CounterForListNodes` (r:1 w:1)
    /// Proof: `VoterList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `VoterList::ListBags` (r:1 w:1)
    /// Proof: `VoterList::ListBags` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
    fn put_in_front_of() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1772`
        //  Estimated: `11506`
        // Minimum execution time: 63_270_000 picoseconds.
        Weight::from_parts(64_710_000, 11506)
            .saturating_add(T::DbWeight::get().reads(10_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
}