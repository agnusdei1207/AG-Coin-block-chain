// [파일 역할]: 템플릿 팔렛 단위 테스트를 위한 목(Mock) 런타임 환경 구성
// [주요 기능]: 테스트용 Test 런타임 구조체 선언, System 및 Template 팔렛 설정, 테스트 익스터널리티(TestExternalities) 초기화

use crate as pallet_template;
use frame_support::derive_impl;
use sp_runtime::BuildStorage;

// [타입 정의]: 테스트용 목 블록 타입
type Block = frame_system::mocking::MockBlock<Test>;

// [테스트 런타임 조립]: System과 Template 팔렛을 포함하는 최소 테스트 런타임 정의
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
	pub struct Test;

	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<Test>;

	#[runtime::pallet_index(1)]
	pub type Template = pallet_template::Pallet<Test>;
}

// [프레임 시스템 설정]: 테스트용 기본 Config 상속
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

// [템플릿 팔렛 설정]: 테스트 환경용 이벤트 및 더미 WeightInfo 적용
impl pallet_template::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

// [목적 / 효과]: 제네시스 스토리지를 빌드하여 테스트 실행 환경(TestExternalities) 인스턴스 생성
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

