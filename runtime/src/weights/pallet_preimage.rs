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

//! Autogenerated weights for `pallet_preimage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `90461ef96cfa`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-preimage
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
// /data/benchmark/runtime/src/weights/pallet_preimage.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.eJYai13mfL

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_preimage` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_preimage::WeightInfo for ZKVWeight<T> {
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::PreimageFor` (r:0 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_preimage(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `114`
        //  Estimated: `3586`
        // Minimum execution time: 40_175_000 picoseconds.
        Weight::from_parts(40_656_000, 3586)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(2_477, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::PreimageFor` (r:0 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_requested_preimage(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 12_273_000 picoseconds.
        Weight::from_parts(12_584_000, 3556)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(2_472, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::PreimageFor` (r:0 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 4194304]`.
    fn note_no_deposit_preimage(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 12_043_000 picoseconds.
        Weight::from_parts(12_323_000, 3556)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(2_483, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::PreimageFor` (r:0 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
    fn unnote_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `316`
        //  Estimated: `3586`
        // Minimum execution time: 38_682_000 picoseconds.
        Weight::from_parts(39_484_000, 3586)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::PreimageFor` (r:0 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
    fn unnote_no_deposit_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `216`
        //  Estimated: `3556`
        // Minimum execution time: 15_600_000 picoseconds.
        Weight::from_parts(16_000_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn request_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `260`
        //  Estimated: `3556`
        // Minimum execution time: 12_454_000 picoseconds.
        Weight::from_parts(12_854_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn request_no_deposit_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `216`
        //  Estimated: `3556`
        // Minimum execution time: 9_578_000 picoseconds.
        Weight::from_parts(9_828_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn request_unnoted_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `114`
        //  Estimated: `3556`
        // Minimum execution time: 10_109_000 picoseconds.
        Weight::from_parts(10_370_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn request_requested_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 8_676_000 picoseconds.
        Weight::from_parts(9_027_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::PreimageFor` (r:0 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
    fn unrequest_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `216`
        //  Estimated: `3556`
        // Minimum execution time: 13_395_000 picoseconds.
        Weight::from_parts(13_816_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn unrequest_unnoted_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 8_346_000 picoseconds.
        Weight::from_parts(8_986_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    fn unrequest_multi_referenced_preimage() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `178`
        //  Estimated: `3556`
        // Minimum execution time: 8_697_000 picoseconds.
        Weight::from_parts(9_127_000, 3556)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Preimage::StatusFor` (r:1023 w:1023)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1023 w:1023)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1023 w:1023)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:0 w:1023)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 1024]`.
    fn ensure_updated(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + n * (227 ±0)`
        //  Estimated: `990 + n * (2603 ±0)`
        // Minimum execution time: 46_066_000 picoseconds.
        Weight::from_parts(46_657_000, 990)
            // Standard Error: 51_477
            .saturating_add(Weight::from_parts(48_929_394, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
    }
}
