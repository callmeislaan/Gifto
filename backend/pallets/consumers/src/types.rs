use crate::Config;
use codec::*;
use frame_support::BoundedVec;
use scale_info::TypeInfo;

pub type ConsumerNameType<T> = BoundedVec<u8, <T as Config>::ConsumerNameLimit>;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Consumer<T: Config> {
	pub name: ConsumerNameType<T>,
	id: T::AccountId,
}

impl<T: Config> Consumer<T> {
	pub fn new(name: ConsumerNameType<T>, id: T::AccountId) -> Self {
		Consumer { name, id }
	}

	pub fn name(&self) -> ConsumerNameType<T> {
		self.name.clone()
	}

	pub fn id(&self) -> T::AccountId {
		self.id.clone()
	}
}
