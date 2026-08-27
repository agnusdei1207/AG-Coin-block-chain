// [파일 역할]: 템플릿 팔렛 성능 측정을 위한 벤치마크 (FRAME v2)
// [주요 기능]: `do_something`, `cause_error` 디스패처 호출에 대한 벤치마크 테스트 케이스 및 가중치 측정 구현

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

// [벤치마크 모듈]: FRAME v2 매크로를 사용한 실행 시간 및 스토리지 I/O 비용 측정 모듈
#[benchmarks]
mod benchmarks {
	use super::*;

	// [벤치마크 항목]: `do_something` 디스패처 실행 시간 및 스토리지 쓰기 비용 측정
	#[benchmark]
	fn do_something() {
		let value = 100u32;
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		do_something(RawOrigin::Signed(caller), value);

		// 실행 후 스토리지에 정상 반영되었는지 확인
		assert_eq!(Something::<T>::get(), Some(value));
	}

	// [벤치마크 항목]: `cause_error` 디스패처의 스토리지 읽기/쓰기 가산 연산 비용 측정
	#[benchmark]
	fn cause_error() {
		Something::<T>::put(100u32);
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));

		// 실행 후 스토리지 값이 101로 정상 증가되었는지 확인
		assert_eq!(Something::<T>::get(), Some(101u32));
	}

	// [테스트 스위트 생성]: 목 런타임을 이용해 벤치마크 로직의 정상 동작 여부를 검증하는 단위 테스트 자동 생성
	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}

