// [파일 역할]: 노드 벤치마크 실행용 트랜잭션 및 인히어런트 생성 도우미 (Benchmarking Setup)
// [주요 기능]: System Remark 빌더, Balance Transfer 빌더, 벤치마크 서명 트랜잭션 및 타임스탬프 인히어런트 데이터 생성

//! Setup code for [`super::command`] which would otherwise bloat that module.
//!
//! Should only be used for benchmarking as it may break in other contexts.

use crate::service::FullClient;

use runtime::{AccountId, Balance, BalancesCall, SystemCall};
use sc_cli::Result;
use sc_client_api::BlockBackend;
use solochain_template_runtime as runtime;
use sp_core::{Encode, Pair};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{OpaqueExtrinsic, SaturatedConversion};

use std::{sync::Arc, time::Duration};

// [목적 / 효과]: 블록 실행 오버헤드 측정을 위한 System::remark 트랜잭션 생성기
pub struct RemarkBuilder {
	client: Arc<FullClient>,
}

impl RemarkBuilder {
	/// 지정된 클라이언트로 RemarkBuilder 인스턴스 생성
	pub fn new(client: Arc<FullClient>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder {
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	// [변환 / 데이터 흐름]: 논스(nonce) -> System::remark OpaqueExtrinsic 생성
	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			SystemCall::remark { remark: vec![] }.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

// [목적 / 효과]: 잔액 전송 오버헤드 측정을 위한 Balances::transfer_keep_alive 트랜잭션 생성기
pub struct TransferKeepAliveBuilder {
	client: Arc<FullClient>,
	dest: AccountId,
	value: Balance,
}

impl TransferKeepAliveBuilder {
	/// 대상 계정 및 전송 금액을 지정하여 TransferKeepAliveBuilder 생성
	pub fn new(client: Arc<FullClient>, dest: AccountId, value: Balance) -> Self {
		Self { client, dest, value }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for TransferKeepAliveBuilder {
	fn pallet(&self) -> &str {
		"balances"
	}

	fn extrinsic(&self) -> &str {
		"transfer_keep_alive"
	}

	// [변환 / 데이터 흐름]: 논스(nonce) -> Balances::transfer_keep_alive OpaqueExtrinsic 생성
	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			BalancesCall::transfer_keep_alive { dest: self.dest.clone().into(), value: self.value }
				.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

// [헬퍼 함수]: 서명자 키쌍, 런타임 콜(Call), 논스를 받아 서명된 UncheckedExtrinsic을 조립
// [목적 / 효과]: 벤치마크 시뮬레이션용 유효한 서명 트랜잭션 페이로드 생성
pub fn create_benchmark_extrinsic(
	client: &FullClient,
	sender: sp_core::sr25519::Pair,
	call: runtime::RuntimeCall,
	nonce: u32,
) -> runtime::UncheckedExtrinsic {
	// 1. 체인 상태(제네시스 해시, 베스트 블록 해시/번호) 추출
	let genesis_hash = client.block_hash(0).ok().flatten().expect("Genesis block exists; qed");
	let best_hash = client.chain_info().best_hash;
	let best_block = client.chain_info().best_number;

	// 2. 트랜잭션 확장(TxExtension) 파이프라인 구성 (수명주기, 논스, 가스비, 메타데이터 해시 등)
	let period = runtime::configs::BlockHashCount::get()
		.checked_next_power_of_two()
		.map(|c| c / 2)
		.unwrap_or(2) as u64;
	let tx_ext: runtime::TxExtension = (
		frame_system::CheckNonZeroSender::<runtime::Runtime>::new(),
		frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
		frame_system::CheckTxVersion::<runtime::Runtime>::new(),
		frame_system::CheckGenesis::<runtime::Runtime>::new(),
		frame_system::CheckEra::<runtime::Runtime>::from(sp_runtime::generic::Era::mortal(
			period,
			best_block.saturated_into(),
		)),
		frame_system::CheckNonce::<runtime::Runtime>::from(nonce),
		frame_system::CheckWeight::<runtime::Runtime>::new(),
		pallet_transaction_payment::ChargeTransactionPayment::<runtime::Runtime>::from(0),
		frame_metadata_hash_extension::CheckMetadataHash::<runtime::Runtime>::new(false),
		frame_system::WeightReclaim::<runtime::Runtime>::new(),
	);

	// 3. 서명 대상 원시 페이로드 생성 및 발신자 키쌍으로 암호학적 서명 수행
	let raw_payload = runtime::SignedPayload::from_raw(
		call.clone(),
		tx_ext.clone(),
		(
			(),
			runtime::VERSION.spec_version,
			runtime::VERSION.transaction_version,
			genesis_hash,
			best_hash,
			(),
			(),
			(),
			None,
			(),
		),
	);
	let signature = raw_payload.using_encoded(|e| sender.sign(e));

	// 4. 서명된 최종 UncheckedExtrinsic 반환
	runtime::UncheckedExtrinsic::new_signed(
		call,
		sp_runtime::AccountId32::from(sender.public()).into(),
		runtime::Signature::Sr25519(signature),
		tx_ext,
	)
}

// [목적 / 효과]: 벤치마크 실행에 필요한 타임스탬프 인히어런트(InherentData) 데이터 생성
pub fn inherent_benchmark_data() -> Result<InherentData> {
	let mut inherent_data = InherentData::new();
	let d = Duration::from_millis(0);
	let timestamp = sp_timestamp::InherentDataProvider::new(d.into());

	futures::executor::block_on(timestamp.provide_inherent_data(&mut inherent_data))
		.map_err(|e| format!("creating inherent data: {:?}", e))?;
	Ok(inherent_data)
}

