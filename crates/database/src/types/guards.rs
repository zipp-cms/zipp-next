use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Valid<T>(T);

impl<T> Valid<T> {
	pub fn assume_valid(v: T) -> Self {
		Self(v)
	}

	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<T> Deref for Valid<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for Valid<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
