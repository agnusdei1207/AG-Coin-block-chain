// [파일 역할]: AG-Coin 블록체인 런타임 최상위 집합체 (Runtime Aggregator)
// [주요 기능]: 기본 통화 단위 및 블록 타임 정의, 트랜잭션 타입 구성, FRAME 팔렛 통합 및 Executive 런타임 조립

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// [모듈 구성]: 런타임 API, 벤치마크, 팔렛 환경설정, 제네시스 프리셋 모듈 선언
pub mod apis;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
pub mod configs;

extern crate alloc;
use alloc::vec::Vec;
use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiAddress, MultiSignature,
};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub mod genesis_config_presets;

// [오파크 타입 정의]: CLI 노드 측에서 내부 세부 구현을 몰라도 블록/헤더를 다룰 수 있도록 캡슐화한 타입 모음
pub mod opaque {
	use super::*;
	use sp_runtime::{
		generic,
		traits::{BlakeTwo256, Hash as HashT},
	};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// 오파크 블록 헤더 타입
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// 오파크 블록 타입
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// 오파크 블록 식별자 타입
	pub type BlockId = generic::BlockId<Block>;
	/// 오파크 블록 해시 타입
	pub type Hash = <BlakeTwo256 as HashT>::Output;
}

// [합의 세션 키]: AURA 블록 생성 키 및 GRANDPA 블록 확정 키 묶음
impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
		pub grandpa: Grandpa,
	}
}

// [런타임 버전 정보]: 온체인 Wasm 업그레이드 호환성 검증을 위한 버전 규격
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("solochain-template-runtime"),
	impl_name: alloc::borrow::Cow::Borrowed("solochain-template-runtime"),
	authoring_version: 1,
	spec_version: 100,
	impl_version: 1,
	apis: apis::RUNTIME_API_VERSIONS,
	transaction_version: 1,
	system_version: 1,
};

// [블록 타임 상수]: 평균 블록 생성 주기 정의 (6초)
mod block_times {
	/// 평균 블록 생성 주기: 6,000 밀리초 (6초)
	pub const MILLI_SECS_PER_BLOCK: u64 = 6000;

	/// AURA 합의 슬롯 주기 (블록당 6초)
	pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;
}
pub use block_times::*;

// [시간 단위 변환 상수]: 블록 수 단위로 환산한 분/시간/일
pub const MINUTES: BlockNumber = 60_000 / (MILLI_SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// 스토리지에 보관할 최근 블록 해시 수
pub const BLOCK_HASH_COUNT: BlockNumber = 2400;

// [통화 단위 상수 (AG-Coin)]: 1 UNIT = 10^12 indivisible units
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLI_UNIT: Balance = 1_000_000_000;
pub const MICRO_UNIT: Balance = 1_000_000;

/// 계정 활성화를 유지하기 위한 최소 잔액 (Existential Deposit)
pub const EXISTENTIAL_DEPOSIT: Balance = MILLI_UNIT;

// [네이티브 버전 정보]: 네이티브 빌드 시 사용되는 버전 메타데이터
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

// [핵심 기본 타입 별칭]
/// 트랜잭션 다중 서명 타입 (Sr25519, Ed25519 등)
pub type Signature = MultiSignature;

/// 온체인 사용자 계정 식별자 (32바이트 공개키)
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// 계정 잔액 타입 (u128)
pub type Balance = u128;

/// 트랜잭션 순번 (논스 Nonce)
pub type Nonce = u32;

/// 블록 및 데이터 해시 타입 (256비트 H256)
pub type Hash = sp_core::H256;

/// 블록 번호 인덱스 타입 (u32)
pub type BlockNumber = u32;

/// 계정 주소 형식
pub type Address = MultiAddress<AccountId, ()>;

/// 런타임 표준 블록 헤더 타입
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// 런타임 표준 블록 타입
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// GRANDPA 확정 증명이 포함된 서명 블록 타입
pub type SignedBlock = generic::SignedBlock<Block>;

/// 블록 고유 식별자 타입
pub type BlockId = generic::BlockId<Block>;

// [트랜잭션 확장 파이프라인 TxExtension]: 트랜잭션 서명 검증, 논스 확인, 가스비 징수, 오버헤드 정산 파이프라인
pub type TxExtension = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
	frame_system::WeightReclaim<Runtime>,
);

/// 런타임 표준 미검증 트랜잭션(Extrinsic) 타입
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;

/// 트랜잭션 서명 대상 페이로드 타입
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

/// 팔렛 외 런타임 업그레이드 시 실행할 마이그레이션 목록
#[allow(unused_parens)]
type Migrations = ();

// [런타임 실행기 Executive]: 블록 초기화, 트랜잭션 적용, 블록 마무리를 지휘하는 최상위 디스패처
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;

// [런타임 합성 Macro]: 개별 FRAME 팔렛들을 묶어 최종 Runtime 인스턴스로 합성
#[frame_support::runtime]
mod runtime {
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Runtime;

	// [팔렛 인덱스 0]: 프레임 시스템 팔렛 (계정, 논스, 블록 헤더 관리)
	#[runtime::pallet_index(0)]
	pub type System = frame_system;

	// [팔렛 인덱스 1]: 블록 온체인 시간 관리 팔렛
	#[runtime::pallet_index(1)]
	pub type Timestamp = pallet_timestamp;

	// [팔렛 인덱스 2]: AURA 블록 생성 합의 팔렛
	#[runtime::pallet_index(2)]
	pub type Aura = pallet_aura;

	// [팔렛 인덱스 3]: GRANDPA 블록 최종 확정 합의 팔렛
	#[runtime::pallet_index(3)]
	pub type Grandpa = pallet_grandpa;

	// [팔렛 인덱스 4]: AG-Coin 토큰 잔액 및 송금 관리 팔렛
	#[runtime::pallet_index(4)]
	pub type Balances = pallet_balances;

	// [팔렛 인덱스 5]: 트랜잭션 수수료(가스비) 정산 팔렛
	#[runtime::pallet_index(5)]
	pub type TransactionPayment = pallet_transaction_payment;

	// [팔렛 인덱스 6]: 루트 관리자 권한 실행 팔렛 (Sudo)
	#[runtime::pallet_index(6)]
	pub type Sudo = pallet_sudo;

	// [팔렛 인덱스 7]: 커스텀 비즈니스 로직 템플릿 팔렛
	#[runtime::pallet_index(7)]
	pub type Template = pallet_template;
}

