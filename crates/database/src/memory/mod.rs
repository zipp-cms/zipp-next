use std::{
	collections::BTreeMap,
	hash::Hash,
	marker::PhantomData,
	sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug, Clone, Copy)]
pub struct Connection<'a> {
	inner: PhantomData<&'a ()>,
}

impl<'a> Connection<'a> {
	pub(super) fn new() -> Self {
		Self { inner: PhantomData }
	}
}

#[derive(Debug)]
pub struct Table<K, V> {
	inner: BTreeMap<K, V>,
}

impl<K, V> Table<K, V>
where
	K: Ord + Eq + Hash,
{
	pub fn new() -> Self {
		Self {
			inner: BTreeMap::new(),
		}
	}

	pub fn get(&self, key: &K) -> Option<&V> {
		self.inner.get(key)
	}

	pub fn find<F>(&self, f: F) -> Option<&V>
	where
		F: Fn(&V) -> bool,
	{
		self.inner.values().find(|v| f(v))
	}

	pub fn any<F>(&self, f: F) -> bool
	where
		F: Fn(&V) -> bool,
	{
		self.inner.values().any(f)
	}

	pub fn all<F>(&self, f: F) -> bool
	where
		F: Fn(&V) -> bool,
	{
		self.inner.values().all(f)
	}

	/// Returns an error if the value already exists
	pub fn insert(&mut self, key: K, value: V) -> Result<(), AlreadyExists> {
		if self.inner.contains_key(&key) {
			return Err(AlreadyExists);
		}

		self.inner.insert(key, value);

		Ok(())
	}
}

#[derive(Debug)]
pub struct AlreadyExists;

#[derive(Debug)]
pub struct ReadWrite<T> {
	inner: Arc<RwLock<T>>,
}

impl<T> ReadWrite<T> {
	pub fn new(inner: T) -> Self {
		Self {
			inner: Arc::new(RwLock::new(inner)),
		}
	}

	pub fn read(&self) -> RwLockReadGuard<'_, T> {
		self.inner.read().unwrap()
	}

	pub fn write(&self) -> RwLockWriteGuard<'_, T> {
		self.inner.write().unwrap()
	}
}

impl<T> Clone for ReadWrite<T> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
		}
	}
}
