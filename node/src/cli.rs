// [파일 역할]: 노드 명령행 인자(CLI) 정의 및 서브커맨드 파서
// [주요 기능]: 노드 구동(Run), 키 관리(Key), 체인 규격 생성(BuildSpec), 블록/상태 가져오기 및 내보내기, 체인 초기화(PurgeChain), 벤치마크 등의 CLI 명령 정의

use sc_cli::RunCmd;

// [목적 / 효과]: 터미널 입력 명령행 인자 파싱 구조체
#[derive(Debug, clap::Parser)]
pub struct Cli {
	// [옵션]: 실행할 서브커맨드 (지정하지 않을 경우 기본 노드 실행으로 동작)
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	// [플래그]: 기본 노드 실행 시 적용되는 공통 설정 (포트, RPC, 데이터 디렉터리 경로 등)
	#[clap(flatten)]
	pub run: RunCmd,
}

// [목적 / 효과]: 노드에서 지원하는 서브커맨드 열거형 목록
#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
	/// 키 관리 관련 유틸리티 명령 (키 생성, 주소 검사, 서명 등)
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// 체인 제네시스 설정 파일(Chain Specification) 생성
	BuildSpec(sc_cli::BuildSpecCmd),

	/// 블록 데이터 유효성 검증
	CheckBlock(sc_cli::CheckBlockCmd),

	/// 체인 데이터베이스에서 블록 파일 내보내기
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// 특정 블록 시점의 체인 상태를 체인 규격(JSON) 형태로 내보내기
	ExportState(sc_cli::ExportStateCmd),

	/// 외부 블록 파일에서 로컬 데이터베이스로 블록 가져오기
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// 로컬 블록체인 데이터베이스 전체 삭제 및 초기화
	PurgeChain(sc_cli::PurgeChainCmd),

	/// 블록체인을 특정 이전 블록 상태로 되돌리기 (Revert)
	Revert(sc_cli::RevertCmd),

	/// 런타임 및 스토리지 성능 측정을 위한 벤치마크 명령
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// 데이터베이스 메타 컬럼 및 스토리지 정보 출력
	ChainInfo(sc_cli::ChainInfoCmd),
}

