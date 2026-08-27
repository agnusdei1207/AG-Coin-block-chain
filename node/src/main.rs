// [파일 역할]: AG-Coin 블록체인 노드 CLI 실행 진입점 (main)
// [주요 기능]: 노드 명령행 인터페이스(CLI) 파싱 및 서브커맨드/노드 서비스 실행 트리거

//! Substrate Node Template CLI library.
#![warn(missing_docs)]

// [모듈 구성]: 노드 실행 및 관리를 위한 서브모듈 선언
mod benchmarking;
mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

// [출처 / 동작 주체]: 노드 프로세스 진입 함수
// [목적 / 효과]: 명령행 인자를 분석하여 체인 동기화, 블록 생성, 벤치마크 등의 서브커맨드 구동
fn main() -> sc_cli::Result<()> {
	command::run()
}

