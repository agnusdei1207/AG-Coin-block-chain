// [파일 역할]: 노드 RPC(Remote Procedure Call) 모듈 확장 및 등록
// [주요 기능]: System RPC(논스 조회 등) 및 Transaction Payment RPC(수수료 계산 등) API 확장 모듈 구성

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use sc_transaction_pool_api::TransactionPool;
use solochain_template_runtime::{opaque::Block, AccountId, Balance, Nonce};
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

// [구성 요소 / 의존성]: 전체 RPC 생성을 위한 클라이언트 및 트랜잭션 풀 의존성
pub struct FullDeps<C, P> {
	/// 노드 블록체인 런타임 클라이언트 인스턴스
	pub client: Arc<C>,
	/// 메모리 내 대기 중인 트랜잭션을 관리하는 트랜잭션 풀 인스턴스
	pub pool: Arc<P>,
}

// [목적 / 효과]: 전체 RPC 확장 모듈(System, TransactionPayment)을 생성하고 통합된 RpcModule 반환
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	// 1. 빈 jsonrpsee RPC 모듈 인스턴스 생성
	let mut module = RpcModule::new(());
	let FullDeps { client, pool } = deps;

	// 2. 시스템 RPC 등록 (계정 논스 조회, 트랜잭션 제출 등)
	module.merge(System::new(client.clone(), pool).into_rpc())?;

	// 3. 트랜잭션 결제 RPC 등록 (예상 가스비/수수료 쿼리)
	module.merge(TransactionPayment::new(client).into_rpc())?;

	Ok(module)
}

