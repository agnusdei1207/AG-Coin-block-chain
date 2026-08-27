// [파일 역할]: 제네시스 블록 초기 상태 프리셋(Genesis Config Presets) 빌더
// [주요 기능]: 개발용(Development) 및 로컬 테스트넷(Local Testnet) 초기 계정 잔액, AURA/GRANDPA 권한자, Sudo 키 설정 JSON 패치 생성


use crate::{AccountId, BalancesConfig, RuntimeGenesisConfig, SudoConfig};
use alloc::{vec, vec::Vec};
use frame_support::build_struct_json_patch;
use serde_json::Value;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_genesis_builder::{self, PresetId};
use sp_keyring::Sr25519Keyring;

// [헬퍼 함수]: 주어진 검증인 권한, 초기 잔액 보유 계정, Sudo 루트 계정으로 RuntimeGenesisConfig JSON 패치 생성
fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
	root: AccountId,
) -> Value {
	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			// 초기 부여 계정별로 1 << 60 잔액 할당
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1u128 << 60))
				.collect::<Vec<_>>(),
		},
		aura: pallet_aura::GenesisConfig {
			// AURA 블록 생성 권한자 목록 등록
			authorities: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},
		grandpa: pallet_grandpa::GenesisConfig {
			// GRANDPA 블록 확정 권한자 목록 등록 (가중치 1)
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>(),
		},
		sudo: SudoConfig { key: Some(root) },
	})
}

// [목적 / 효과]: 개발용(Development) 단일 노드 제네시스 설정 반환 (Alice 단독 검증인 및 Sudo)
pub fn development_config_genesis() -> Value {
	testnet_genesis(
		// 1. 단일 검증인(Alice) 등록
		vec![(
			sp_keyring::Sr25519Keyring::Alice.public().into(),
			sp_keyring::Ed25519Keyring::Alice.public().into(),
		)],
		// 2. 초기 잔액 부여 계정 목록 (Alice, Bob, AliceStash, BobStash)
		vec![
			Sr25519Keyring::Alice.to_account_id(),
			Sr25519Keyring::Bob.to_account_id(),
			Sr25519Keyring::AliceStash.to_account_id(),
			Sr25519Keyring::BobStash.to_account_id(),
		],
		// 3. Sudo 관리자 계정: Alice
		sp_keyring::Sr25519Keyring::Alice.to_account_id(),
	)
}

// [목적 / 효과]: 로컬 테스트넷(Local Testnet) 멀티 노드 제네시스 설정 반환 (Alice, Bob 듀얼 검증인)
pub fn local_config_genesis() -> Value {
	testnet_genesis(
		// 1. 복수 검증인(Alice, Bob) 등록
		vec![
			(
				sp_keyring::Sr25519Keyring::Alice.public().into(),
				sp_keyring::Ed25519Keyring::Alice.public().into(),
			),
			(
				sp_keyring::Sr25519Keyring::Bob.public().into(),
				sp_keyring::Ed25519Keyring::Bob.public().into(),
			),
		],
		// 2. 잘 알려진 테스트 계정 목록 전체에 초기 잔액 부여
		Sr25519Keyring::iter()
			.filter(|v| v != &Sr25519Keyring::One && v != &Sr25519Keyring::Two)
			.map(|v| v.to_account_id())
			.collect::<Vec<_>>(),
		// 3. Sudo 관리자 계정: Alice
		Sr25519Keyring::Alice.to_account_id(),
	)
}

// [변환 / 데이터 흐름]: PresetId -> 직렬화된 제네시스 설정 JSON 바이트 벡터
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

// [목적 / 효과]: 지원하는 제네시스 프리셋 이름 목록 반환
pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
	]
}
