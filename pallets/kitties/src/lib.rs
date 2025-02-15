#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod migrations;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*, Blake2_128Concat, PalletId, traits::ExistenceRequirement};
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, Randomness};
    use sp_io::hashing::blake2_128;
    use sp_runtime::traits::AccountIdConversion;
    use crate::migrations;
    pub type KittyId = u32;
    pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    #[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]

    pub struct Kitty{
        pub dna: [u8; 16],
        pub name: [u8; 8],
    }
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);


    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type representing the weight of this pallet
        type WeightInfo: WeightInfo;
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        type Currency: Currency<Self::AccountId>;
        #[pallet::constant]
        type KittyPrice: Get<BalanceOf<Self>>;
        type PalletId: Get<PalletId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_parents)]
    pub type KittyParents<T: Config> =
    StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_on_sale)]
    pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, ()>;
    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
        KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
        KittyTransferred { who: T::AccountId, to: T::AccountId, kitty_id: KittyId },
        KittyOnSale { who: T::AccountId, kitty_id: KittyId },
        KittyBought {who: T::AccountId, kitty_id:KittyId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        InvalidKittyId,
        SameKittyId,
        NotOwner,
        AlreadyOnSale,
        AlreadyOwned,
        NotOnSale,
        NoOwner,
    }

    #[pallet::hooks]
    impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            migrations::v2::migrate::<T>()
        }
    }
    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(origin: OriginFor<T>, name: [u8; 8]) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty_id = Self::get_next_id()?;
            let dna = Self::random_value(&who);
            let kitty = Kitty {dna, name};

            let price = T::KittyPrice::get();
            // T::Currency::reserve(&who, price)?;
            T::Currency::transfer(&who, &Self::get_account_id(), price, ExistenceRequirement::KeepAlive)?;

            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);

            // Emit an event.
            Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(
            origin: OriginFor<T>,
            kitty_id_1: KittyId,
            kitty_id_2: KittyId,
            name: [u8; 8],
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

            ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
            ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);

            let kitty_id = Self::get_next_id()?;
            let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
            let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

            let selector = Self::random_value(&who);
            let dna = [0u8; 16];
            // for i in 0..kitty_1.0.len() {
            // 	data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
            // }

            let kitty = Kitty{dna, name};

            let price = T::KittyPrice::get();
            // T::Currency::reserve(&who, price)?;
            T::Currency::transfer(&who, &Self::get_account_id(), price, ExistenceRequirement::KeepAlive)?;

            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);
            KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

            Self::deposit_event(Event::KittyBred { who, kitty_id, kitty });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, recepient: T::AccountId, kitty_id: KittyId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(KittyOwner::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

            let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
            ensure!(owner == who, Error::<T>::NotOwner);

            KittyOwner::<T>::insert(kitty_id, &recepient);
            Self::deposit_event(Event::KittyTransferred{who, to: recepient, kitty_id});

            Ok(())
        }
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(origin: OriginFor<T>, kitty_id: u32, _price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
            let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::NoOwner)?;
            ensure!(owner == who, Error::<T>::NotOwner);
            ensure!(!KittyOnSale::<T>::contains_key(kitty_id), Error::<T>::AlreadyOnSale);

            KittyOnSale::<T>::insert(kitty_id, ());

            Self::deposit_event(Event::KittyOnSale { who, kitty_id });

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::buy())]
        pub fn buy(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::kitties(kitty_id).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;

            let owner =
                Self::kitty_owner(kitty_id).ok_or::<DispatchError>(Error::<T>::NoOwner.into())?;
            ensure!(owner != who, Error::<T>::AlreadyOwned);
            ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::NotOnSale);

            let price = T::KittyPrice::get();
            // T::Currency::reserve(&who, price)?;
            // T::Currency::unreserve(&owner, price);
            T::Currency::transfer(&who, &owner, price, ExistenceRequirement::KeepAlive)?;


            <KittyOwner<T>>::insert(kitty_id, &who);
            <KittyOnSale<T>>::remove(kitty_id);

            Self::deposit_event(Event::KittyBought { who, kitty_id });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn get_next_id() -> Result<KittyId, DispatchError> {
            NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id
                    .checked_add(1)
                    .ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                Ok(current_id)
            })
        }

        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }
        fn get_account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }
    }
}