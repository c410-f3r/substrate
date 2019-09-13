use support::{decl_module, decl_storage, StorageValue};

pub trait Trait: system::Trait {
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		OddEvenCounter: u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
	}
}

impl<T: Trait> Module<T> {
	pub fn increase_odd_even_counter() {
		OddEvenCounter::put(OddEvenCounter::get() + 1);
	}

	pub fn odd_even_counter() -> u32 {
		OddEvenCounter::get()
	}
}
