// [파일 역할]: 런타임 Wasm 바이너리 빌드 스크립트 (Wasm Builder)
// [주요 기능]: 온체인 실행을 위한 Substrate Wasm 런타임 컴파일 및 메타데이터 해시 생성 설정

// [출처 / 동작 주체]: std 및 metadata-hash 활성화 시 메타데이터 해시가 포함된 Wasm 바이너리 생성
#[cfg(all(feature = "std", feature = "metadata-hash"))]
fn main() {
	substrate_wasm_builder::WasmBuilder::init_with_defaults()
		.enable_metadata_hash("UNIT", 12)
		.build();
}

// [출처 / 동작 주체]: 기본 std 빌드 시 표준 설정으로 Wasm 바이너리 생성
#[cfg(all(feature = "std", not(feature = "metadata-hash")))]
fn main() {
	substrate_wasm_builder::WasmBuilder::build_using_defaults();
}

// [목적 / 효과]: Wasm 자체 컴파일 시 불필요한 재귀 빌드 방지를 위한 no-op 진입점
#[cfg(not(feature = "std"))]
fn main() {}

