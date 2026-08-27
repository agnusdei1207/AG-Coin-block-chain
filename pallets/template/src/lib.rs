// [파일 역할]: AG-Coin 템플릿 팔렛 (Template Pallet) 구현체
// [주요 기능]: 스토리지 값 저장(`do_something`), 값 증가 및 에러 처리(`cause_error`), 이벤트 및 에러 정의

//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.

#![cfg_attr(not(feature = "std"), no_std)]

// [모듈 재내보내기]: 팔렛 항목들을 크레이트 네임스페이스로 공개
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// [프레임 팔렛 선언]: FRAME 매크로 기반 팔렛 모듈 정의
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// [목적 / 효과]: 팔렛 트레이트와 디스패처 메서드를 구현하기 위한 기본 구조체
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// [구성 트레이트 Config]: 팔렛이 의존하는 런타임 타입 및 가중치 정보 정의
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// 전체 런타임 이벤트 타입
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// 팔렛 디스패처 호출에 필요한 가중치(Weight) 계산 인터페이스
		type WeightInfo: WeightInfo;
	}

	// [상태 저장소 Storage]: 단일 u32 정수값을 보관하는 온체인 스토리지 항목
	#[pallet::storage]
	pub type Something<T> = StorageValue<_, u32>;

	// [이벤트 정의 Event]: 상태 변경 발생 시 외부 클라이언트 및 탐색기에 전파하는 이벤트 목록
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// 사용자가 새로운 값을 성공적으로 스토리지에 저장했을 때 발생하는 이벤트
		SomethingStored {
			/// 저장된 새로운 u32 값
			something: u32,
			/// 값을 설정한 서명자 계정
			who: T::AccountId,
		},
	}

	// [에러 정의 Error]: 디스패처 호출 실패 시 반환되는 런타임 에러 목록
	#[pallet::error]
	pub enum Error<T> {
		/// 스토리지에 값이 설정되어 있지 않아 읽을 수 없음
		NoneValue,
		/// 스토리지 값 1 증가 시 u32 오버플로우가 발생함
		StorageOverflow,
	}

	// [호출 함수 (Extrinsics / Dispatchables)]: 사용자가 서명 트랜잭션으로 호출 가능한 외부 함수 목록
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// [목적 / 효과]: 스토리지에 전달받은 u32 값을 직접 저장하고 `SomethingStored` 이벤트를 방출
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// 1. 호출자 서명 검증 및 계정 ID 획득
			let who = ensure_signed(origin)?;

			// 2. 온체인 스토리지에 새로운 값 기록
			Something::<T>::put(something);

			// 3. 상태 변경 이벤트 방출
			Self::deposit_event(Event::SomethingStored { something, who });

			// 4. 성공 결과 반환
			Ok(())
		}

		// [목적 / 효과]: 기존 스토리지 값을 읽어 1을 더한 후 다시 저장하며, 비어있거나 오버플로우 발생 시 에러 반환
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			// 1. 호출자 서명 검증
			let _who = ensure_signed(origin)?;

			// 2. 기존 스토리지 값 확인 및 가산 처리
			match Something::<T>::get() {
				// [실패 1]: 스토리지에 값이 존재하지 않는 경우
				None => Err(Error::<T>::NoneValue.into()),
				// [성공/가산]: 값이 존재할 때 1 증가 후 저장
				Some(old) => {
					// [실패 2]: u32 최대값 초과로 오버플로우 발생 시 에러 반환
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// 새로운 값으로 스토리지 갱신
					Something::<T>::put(new);
					Ok(())
				},
			}
		}
	}
}

