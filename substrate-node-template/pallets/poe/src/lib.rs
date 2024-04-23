// 编译标签————不使用std标签编译的话，就使用no_std标签
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// frame_support::pallet宏————
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*; /* 引入预定义依赖，比如Get接口、常用宏 */
	use frame_system::pallet_prelude::*; // 包含ensure_signed，ensure_none这样的签名验证方法

	// trait Config继承了frame_system::Config的一些数据类型，比如BlockNumber(表示数据块数量)，
	// hash表示hash类型，AccountID表示用户ID
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// 最大存证长度，如果存证过长会导致链上状态爆炸，
		// 通常在链上只存储原始内容的哈希值(哈希值长度固定)
		#[pallet::constant] // MaxClaimLength是常量，所以需要使用这个宏来声明MaxClaimLength是一个链上常量
		type MaxClaimLength: Get<u32>; // Get接口定义的u32，存储项BoundedVec中使用
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	// #[pallet::generate_store(pub(super) trait Store)] // 因为需要存储项，所以这里使用generate_store宏
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	// StorageMap表示一个键值对
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat, /* 一种hash算法，用来将存储项存储到数据库的时候，
		                   * 对它的存储位置进行hash计算 */
		BoundedVec<u8, T::MaxClaimLength>, // key
		(T::AccountId, T::BlockNumber),     /* value，AccountId表示属于哪个用户，
		                                     * BlockNumber表示在哪一个区块存储进链的 */
	>;

	#[pallet::event]
	#[pallet::generate_deposit (pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>), // 创建凭证
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>), // 注销凭证
		ClaimTransfered(T::AccountId, BoundedVec<u8, T::MaxClaimLength>), // 转移凭证
	}

	// 定义错误类型
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
		CanNotTransferToSelf,
	}

	// 定义保留函数，但因为存证模块不需要使用保留函数，这里置空。保留函数通常包括on_finalize、
	// on_runtime_upgrade等，在区块的不同时机执行。
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// 可调用函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)] // 定义可调函数在模块里的顺序
		#[pallet::weight(0)] // 给调用函数指定权重
		pub fn create_claim(
			origin: OriginFor<T>, // origin表示交易的发送方，claim是存证内容
			claim: BoundedVec<u8, T::MaxClaimLength>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 校验交易发送方(交易是一笔签名的交易)
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist); // 校验凭证是否已存在
			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?; // 校验交易发送方(交易是一笔签名的交易)
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?; // 获取存证的所有者
            ensure!(owner == sender, Error::<T>::NotClaimOwner); // 检查交易发起方是否是存证的所有者，不是所有者不能注销存证
            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

		#[pallet::call_index(2)]
        #[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			to: T::AccountId,
			claim: BoundedVec<u8, T::MaxClaimLength>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 校验交易发送方(交易是一笔签名的交易)
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?; // 获取存证的所有者
			ensure!(owner == sender, Error::<T>::NotClaimOwner); // 检查交易发起方是否是存证的所有者，不是所有者不能转移存证
            ensure!(owner != to, Error::<T>::CanNotTransferToSelf); // 所有者不能把存证转移给自己
			Proofs::<T>::remove(&claim);
			Proofs::<T>::insert(
				&claim,
				(to.clone(), frame_system::Pallet::<T>::block_number()),
			);
			Self::deposit_event(Event::ClaimTransfered(to, claim));
			Ok(().into())
		}
	}
}