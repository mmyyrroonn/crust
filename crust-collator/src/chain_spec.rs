// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use crust_parachain_primitives::{AccountId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_core::crypto::UncheckedInto;
use parachain_runtime::{AuraId};
use parachain_runtime::{Balance, CENTS};

const SHADOW_ED: Balance = 10 * CENTS;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate a crypto pair from seed
pub fn get_pair_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_pair_from_seed::<AuraId>(seed)
}

pub fn crust_shadow_session_keys(keys: AuraId) -> parachain_runtime::opaque::SessionKeys {
	parachain_runtime::opaque::SessionKeys { aura: keys }
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn get_chain_spec(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![(
					hex!("0a38d76ecfbd4b13077669bb9c9ebaaf0847723426f809d20a67c62f2bebc75a").into(),
					hex!("0a38d76ecfbd4b13077669bb9c9ebaaf0847723426f809d20a67c62f2bebc75a").unchecked_into()
				),
				(
					hex!("7e5040d49782960b2a15e7cb4106f730ca7a997a28facff6e2978aeda32fc348").into(),
					hex!("7e5040d49782960b2a15e7cb4106f730ca7a997a28facff6e2978aeda32fc348").unchecked_into()
				),
				(
					hex!("869f4e66b0b16f6de5f3cc217b99ada20f766a7d26868dda3020d29e9e80e97c").into(),
					hex!("869f4e66b0b16f6de5f3cc217b99ada20f766a7d26868dda3020d29e9e80e97c").unchecked_into()
				),
				(
					hex!("7a6a226782a4cf5712f914bbf3cc64304f3c9af58b82f1dd2a4f09c48278ae65").into(),
					hex!("7a6a226782a4cf5712f914bbf3cc64304f3c9af58b82f1dd2a4f09c48278ae65").unchecked_into()
				),
			],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					hex!["0a38d76ecfbd4b13077669bb9c9ebaaf0847723426f809d20a67c62f2bebc75a"].into(),
					hex!["7e5040d49782960b2a15e7cb4106f730ca7a997a28facff6e2978aeda32fc348"].into(),
					hex!["869f4e66b0b16f6de5f3cc217b99ada20f766a7d26868dda3020d29e9e80e97c"].into(),
					hex!["7a6a226782a4cf5712f914bbf3cc64304f3c9af58b82f1dd2a4f09c48278ae65"].into()
				],
				id,
			)
		},
		vec![],
		None,
		None,
		None,
		Extensions {
			relay_chain: "westend-dev".into(),
			para_id: id.into(),
		},
	)
}

pub fn staging_test_net(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
				vec![(
					hex!("0a38d76ecfbd4b13077669bb9c9ebaaf0847723426f809d20a67c62f2bebc75a").into(),
					hex!("0a38d76ecfbd4b13077669bb9c9ebaaf0847723426f809d20a67c62f2bebc75a").unchecked_into()
				),
				(
					hex!("7e5040d49782960b2a15e7cb4106f730ca7a997a28facff6e2978aeda32fc348").into(),
					hex!("7e5040d49782960b2a15e7cb4106f730ca7a997a28facff6e2978aeda32fc348").unchecked_into()
				),
				(
					hex!("869f4e66b0b16f6de5f3cc217b99ada20f766a7d26868dda3020d29e9e80e97c").into(),
					hex!("869f4e66b0b16f6de5f3cc217b99ada20f766a7d26868dda3020d29e9e80e97c").unchecked_into()
				),
				(
					hex!("7a6a226782a4cf5712f914bbf3cc64304f3c9af58b82f1dd2a4f09c48278ae65").into(),
					hex!("7a6a226782a4cf5712f914bbf3cc64304f3c9af58b82f1dd2a4f09c48278ae65").unchecked_into()
				),
			],
				vec![
					hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
					hex!["0a38d76ecfbd4b13077669bb9c9ebaaf0847723426f809d20a67c62f2bebc75a"].into(),
					hex!["7e5040d49782960b2a15e7cb4106f730ca7a997a28facff6e2978aeda32fc348"].into(),
					hex!["869f4e66b0b16f6de5f3cc217b99ada20f766a7d26868dda3020d29e9e80e97c"].into(),
					hex!["7a6a226782a4cf5712f914bbf3cc64304f3c9af58b82f1dd2a4f09c48278ae65"].into()
				],
				id,
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Extensions {
			relay_chain: "westend-dev".into(),
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
	parachain_runtime::GenesisConfig {
		frame_system: parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: parachain_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		market: parachain_runtime::MarketConfig { },
		pallet_sudo: parachain_runtime::SudoConfig { key: root_key },
		parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
		pallet_collator_selection: parachain_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: SHADOW_ED * 16,
			..Default::default()
		},
		pallet_session: parachain_runtime::SessionConfig {
			keys: invulnerables.iter().cloned().map(|(acc, aura)| (
				acc.clone(), // account id
				acc.clone(), // validator id
				crust_shadow_session_keys(aura), // session keys
			)).collect()
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		pallet_aura: Default::default(),
		cumulus_pallet_aura_ext: Default::default(),
		cumulus_pallet_parachain_system: Default::default()
	}
}