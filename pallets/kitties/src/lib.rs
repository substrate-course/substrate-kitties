#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_error, StorageValue, StorageMap, traits::Randomness};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): u32;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err(Error::<T>::KittiesCountOverflow.into());
			}
			let payload = (
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				sender,
				<frame_system::Module<T>>::extrinsic_index(),
			);
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}
	}
}
