#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		// Something get(fn something): Option<u32>;
		// 如果证明有所有者和证件号码，那么我们知道它已被要求保护！否则，可以要求证明。
		Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
		MaxLimit get(fn limit): Option<u32>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		// 索取证明后发出的事件。将新证明添加到区块链时。
		ClaimCreated(AccountId, Vec<u8>),
		// 所有者撤消索赔时发出的事件。移除证明时。
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransfer(AccountId, Vec<u8>, AccountId),
		MaxLimitSet(AccountId, u32),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		// NoneValue,
		/// Value reached maximum and cannot be incremented further
		// StorageOverflow,
		ProofAlreadyClaimed,
		NoSuchProof,
		NoProofOwner,
		OverMaxLimit,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		#[weight = 10_000]
		fn create_claim(origin, proof: Vec<u8>) {
			let sender = ensure_signed(origin)?;
			let limit = match MaxLimit::get() {
				None => {
					let limit: u32 = 10;
					MaxLimit::put(limit);
					Self::deposit_event(RawEvent::MaxLimitSet(sender.clone(), limit));
					limit
				},
				Some(limit) => limit,
			};
			ensure!(proof.len() <= limit as usize, Error::<T>::OverMaxLimit);
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);
			let current_block = <system::Module<T>>::block_number();

			Proofs::<T>::insert(&proof, (&sender, current_block));
			Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
		}

		#[weight = 10_000]
		fn revoke_claim(origin, proof: Vec<u8>) {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _) = Proofs::<T>::get(&proof);

			ensure!(sender == owner, Error::<T>::NoProofOwner);
			Proofs::<T>::remove(&proof);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
		}

		#[weight = 10_000]
		fn transfer_claim(origin, to: T::AccountId, proof: Vec<u8>) {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _) = Proofs::<T>::get(&proof);
			let current_block = <system::Module<T>>::block_number();

			ensure!(sender == owner, Error::<T>::NoProofOwner);
			Proofs::<T>::insert(&proof, (&to, current_block));
			Self::deposit_event(RawEvent::ClaimTransfer(sender, proof, to));
		}

		#[weight = 10_000]
		fn set_limit(origin, limit: u32) {
			let sender = ensure_signed(origin)?;
			MaxLimit::put(limit);
			Self::deposit_event(RawEvent::MaxLimitSet(sender, limit));
		}
	}
}
