// [파일 역할]: 템플릿 팔렛 기능 검증용 단위 테스트 (Unit Tests)
// [주요 기능]: 스토리지 값 설정/이벤트 방출 테스트, 빈 값 접근 시 NoneValue 에러 반환 검증

use crate::{mock::*, Error, Event, Something};
use frame_support::{assert_noop, assert_ok};

// [테스트 목적]: `do_something` 함수 호출 시 스토리지가 갱신되고 올바른 이벤트가 방출되는지 검증
#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// 1. 이벤트 저장을 위해 블록 번호를 1로 설정
		System::set_block_number(1);

		// 2. 계정 ID 1번으로 do_something(42) 서명 트랜잭션 호출 성공 검증
		assert_ok!(Template::do_something(RuntimeOrigin::signed(1), 42));

		// 3. 온체인 스토리지에 42가 정상 저장되었는지 확인
		assert_eq!(Something::<Test>::get(), Some(42));

		// 4. SomethingStored 이벤트가 정상 방출되었는지 확인
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

// [테스트 목적]: 스토리지 값이 없는 상태에서 `cause_error` 호출 시 `Error::NoneValue` 에러를 반환하는지 검증
#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// 1. 스토리지에 값이 없을 때 cause_error 호출 시 NoneValue 에러가 발생하는지 검증
		assert_noop!(Template::cause_error(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
	});
}

