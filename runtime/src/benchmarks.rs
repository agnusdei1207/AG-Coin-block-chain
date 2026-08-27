// [파일 역할]: 런타임 전체 팔렛 대상 벤치마크 매크로 정의 (Define Benchmarks)
// [주요 기능]: Baseline, System, SystemExtensions, Balances, Timestamp, Sudo, Template 팔렛의 벤치마크 스위트 등록

// [벤치마크 등록 매크로]: 각 팔렛별 벤치마크 실행 대상을 런타임에 일괄 등록
frame_benchmarking::define_benchmarks!(
	[frame_benchmarking, BaselineBench::<Runtime>]
	[frame_system, SystemBench::<Runtime>]
	[frame_system_extensions, SystemExtensionsBench::<Runtime>]
	[pallet_balances, Balances]
	[pallet_timestamp, Timestamp]
	[pallet_sudo, Sudo]
	[pallet_template, Template]
);

