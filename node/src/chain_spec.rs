use sp_core::{Pair, Public, sr25519};
use node_template_runtime::{
	AccountId, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig,  Signature, wasm_binary_unwrap, StakerStatus, TokensConfig, AssetRegistryConfig, OracleConfig
};
// Staking related Configs
use node_template_runtime::{
	BabeConfig, ImOnlineConfig, SessionConfig, StakingConfig,
};
use node_template_runtime::constants::currency::{Balance, DOLLARS};
use node_template_runtime::opaque::SessionKeys;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sp_runtime::Perbill;
use sc_service::ChainType;
use serde_json;


// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

const PROPERTIES: &str = r#"
{
	"ss58format": 7,
	"tokenDecimals": 15,
	"tokenSymbol": "STND"
}	
"#;


const PROTOCOL_ID: &str = "stnd";

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate controller and session key from seed
pub fn get_pos_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn session_keys_pos(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys {
		grandpa,
		babe,
		im_online,
		authority_discovery,
	}
}

pub type AssetId = u32;
pub const CORE_ASSET_ID: AssetId = 0;

const STASH: Balance = 100 * DOLLARS;

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = wasm_binary_unwrap().to_vec();
	let prop_map: serde_json::map::Map<std::string::String, serde_json::value::Value> =
	serde_json::from_str(PROPERTIES).map_err(|err|format!("json err:{}",err))?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Standard Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary.clone(),
			// Initial PoA authorities
			vec![
				get_pos_keys_from_seed("Alice"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some(PROTOCOL_ID),
		// Properties
		Some(prop_map),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = wasm_binary_unwrap().to_vec();
	let prop_map: serde_json::map::Map<std::string::String, serde_json::value::Value> =
	serde_json::from_str(PROPERTIES).map_err(|err|format!("json err:{}",err))?;
	Ok(ChainSpec::from_genesis(
		// Name
		"Standard Testnet",
		// ID
		"standard_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary.clone(),
			// Initial PoA authorities
			vec![
				get_pos_keys_from_seed("Alice"),
				get_pos_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
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
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some(PROTOCOL_ID),
		// Properties
		Some(prop_map),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: Vec<u8>,
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary,
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![] 
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		// Staking related configs
		pallet_babe: Some(BabeConfig { authorities: vec![] }),
		//pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
		//pallet_treasury: Some(Default::default()),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys_pos(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		orml_tokens: Some(TokensConfig {
			endowed_accounts: endowed_accounts
				.iter()
				.flat_map(|x| {
					vec![
						(x.clone(), 1, 10_000_000_000_000_000_u128),
						(x.clone(), 2, 50_000_000_000_000_000_u128),
						(x.clone(), 3, 100_000_000_000_000_000_u128),
					]
				})
				.collect(),
		}),
		pallet_asset_registry: Some(AssetRegistryConfig {
			core_asset_id: CORE_ASSET_ID,
			asset_ids: vec![
				(b"STD".to_vec(), 1),
				(b"MTR".to_vec(), 2),
				(b"DOT".to_vec(), 3),
				(b"KSM".to_vec(), 4),

			],
			next_asset_id: 5,
		}),
		pallet_standard_oracle: Some(OracleConfig{
			oracles: [get_account_id_from_seed::<sr25519::Public>("Alice")].to_vec()
        }),
		//pallet_elections_phragmen: Some(ElectionsConfig { members: vec![] }),
		//pallet_collective_Instance1: Some(CouncilConfig::default()),
	}
}