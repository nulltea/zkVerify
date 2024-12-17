#![cfg_attr(not(feature = "std"), no_std)]

mod vk;
mod circuit_info;
mod params;

use std::io::Cursor;
use codec::Encode;
use frame_benchmarking::BenchmarkParameter::r;
use frame_support::weights::Weight;
use halo2_proofs::halo2curves::bn256::{Bn256, G1Affine};
use hp_verifiers::{Cow, Verifier, VerifyError};
use halo2_proofs::plonk::verify_proof;
use halo2_proofs::transcript::{Blake2bRead, Challenge255, TranscriptReadBuffer};
use crate::vk::Fr;
use halo2_proofs::halo2curves::bn256;
use halo2_proofs::poly::kzg::commitment::KZGCommitmentScheme;
use halo2_proofs::poly::kzg::multiopen::VerifierSHPLONK;
use halo2_proofs::poly::kzg::strategy::SingleStrategy;

#[pallet_verifiers::verifier]
pub struct Halo2;

struct Public(Vec<Vec<Vec<Fr>>>);

impl Verifier for Halo2 {
    type Proof = Vec<u8>;

    type Pubs = Vec<Vec<Vec<Fr>>>;

    type Vk = (vk::Vk, params::ParamsKZG);

    fn hash_context_data() -> &'static [u8] {
        b"fflonk"
    }

    fn verify_proof(
        (vk, params): &(vk::Vk, params::ParamsKZG),
        raw_proof: &Self::Proof,
        raw_pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        let vk = vk
            .clone()
            .try_into()
            .map_err(|e| log::debug!("Invalid Vk: {:?}", e))
            .map_err(|_| VerifyError::InvalidVerificationKey)?;
        let params = params
            .clone()
            .try_into()
            .map_err(|e| log::debug!("Invalid Params: {:?}", e))
            .map_err(|_| VerifyError::InvalidVerificationKey)?;
        // let pubs: Vec<Vec<Vec<bn256::Fr>>> = raw_pubs[0].into_iter().map(|x| *x.into_iter().map(|x| *x.into_iter().map(|x|
        //     x.clone().into()
        // ).collect::<Vec<_>>()).collect::<Vec<_>>()).collect();

        let pubs = vec![];

        let mut transcript = Blake2bRead::init(raw_proof.as_slice());

        // log::trace!(
        //     "Extracted public inputs [{:?}...{:?}] and proof data [{:?}...{:?}]",
        //     &raw_pubs[0],
        //     &raw_pubs[PUBS_SIZE - 1],
        //     &raw_proof[0],
        //     &raw_proof[PROOF_SIZE - 1]
        // );

        let strategy = SingleStrategy::new(&params);

        verify_proof::<
            KZGCommitmentScheme<Bn256>,
            VerifierSHPLONK<'_, Bn256>,
            Challenge255<G1Affine>,
            Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
            SingleStrategy<'_, Bn256>,
        >(&params, &vk, strategy, &pubs, &mut transcript)
            .map(|_| ())
            .map_err(|e| log::debug!("Verification failed: {:?}", e))
            .map_err(|_| VerifyError::VerifyError)
    }

    fn validate_vk(vk: &Self::Vk) -> Result<(), hp_verifiers::VerifyError> {
        let _: halo2_proofs::plonk::VerifyingKey<bn256::G1Affine> = vk.0
            .clone()
            .try_into()
            .map_err(|e| log::debug!("Invalid Vk: {:?}", e))
            .map_err(|_| VerifyError::InvalidVerificationKey)?;
        Ok(())
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        Cow::Owned(pubs.encode())
    }
}

impl TryFrom<&[u8]> for Public {
    type Error = core::array::TryFromSliceError;

    fn try_from(inner: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}