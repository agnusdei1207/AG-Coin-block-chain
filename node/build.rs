// [파일 역할]: 노드 빌드 시점 메타데이터 생성 빌드 스크립트 (build.rs)
// [주요 기능]: Cargo 키 및 빌드 환경변수 생성, Git HEAD 변경 시 재컴파일 트리거 설정

use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

// [출처 / 동작 주체]: Cargo 빌드 스크립트 실행 진입점
fn main() {
	// 1. 빌드 버전 정보 및 구현체 메타데이터(SUBSTRATE_CLI_IMPL_VERSION 등)를 Cargo 환경변수로 생성
	generate_cargo_keys();

	// 2. Git 저장소의 HEAD 커밋이 변경되면 Cargo가 자동으로 다시 빌드하도록 설정
	rerun_if_git_head_changed();
}

