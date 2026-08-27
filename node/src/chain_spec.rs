// [파일 역할]: 블록체인 네트워크 규격 및 초기 제네시스 설정 정의 (Chain Spec)
// [주요 기능]: 개발 모드(Development) 및 로컬 테스트넷(Local Testnet) 체인 규격 생성

use sc_service::ChainType;
use solochain_template_runtime::WASM_BINARY;

// [목적 / 효과]: Substrate 일반 체인 규격을 사용하는 특화 타입 별칭
pub type ChainSpec = sc_service::GenericChainSpec;

// [목적 / 효과]: 단일 노드 로컬 개발 환경용 체인 규격(ChainSpec) 생성
// [동작 주체]: 개발용 Wasm 바이너리와 `sp_genesis_builder::DEV_RUNTIME_PRESET` 프리셋을 결합하여 체인 초기화
pub fn development_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		// 1. 런타임 Wasm 바이너리 포함 여부 확인
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
	.build())
}

// [목적 / 효과]: 복수 검증인(Validator) 기반 로컬 테스트넷용 체인 규격(ChainSpec) 생성
// [동작 주체]: `sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET` 프리셋을 적용하여 Alice/Bob 권한 노드 초기화
pub fn local_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		// 1. 런타임 Wasm 바이너리 포함 여부 확인
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
	.build())
}

