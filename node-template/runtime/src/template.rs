use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;
use rstd::prelude::*;
use rstd::collections::btree_set::BTreeSet;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		State: BTreeSet<Vec<u8>>;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn add_bytes(origin, bytes: Vec<u8>) -> Result {
			let who = ensure_signed(origin)?;
			let mut state = <Self as Store>::State::get();
			state.insert(bytes);
			<Self as Store>::State::put(state);
			Self::deposit_event(RawEvent::Stored(who));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		Stored(AccountId),
	}
);

impl<T: Trait> Module<T> {
	pub fn state() -> BTreeSet<Vec<u8>> {
		<Self as Store>::State::get()
	}
}