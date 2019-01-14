use crate::{key::Key, storage::Storage};
use std::mem;

/// Storage for `Option<T>`s.
pub struct OptionStorage<K, V: 'static>
where
    K: Key<K, V>,
{
    some: K::Storage,
    none: Option<V>,
}

impl<K, V> Clone for OptionStorage<K, V>
where
    K: Key<K, V>,
    K::Storage: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        OptionStorage {
            some: self.some.clone(),
            none: self.none.clone(),
        }
    }
}

impl<K, V> Default for OptionStorage<K, V>
where
    K: Key<K, V>,
{
    fn default() -> Self {
        Self {
            some: Default::default(),
            none: Default::default(),
        }
    }
}

impl<K, V> PartialEq for OptionStorage<K, V>
where
    K: Key<K, V>,
    K::Storage: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.none == other.none && self.some == other.some
    }
}

impl<K, V> Eq for OptionStorage<K, V>
where
    K: Key<K, V>,
    K::Storage: Eq,
    V: Eq,
{
}

impl<K, V> Storage<Option<K>, V> for OptionStorage<K, V>
where
    K: Key<K, V>,
{
    #[inline]
    fn insert(&mut self, key: Option<K>, value: V) -> Option<V> {
        match key {
            Some(key) => self.some.insert(key, value),
            None => mem::replace(&mut self.none, Some(value)),
        }
    }

    #[inline]
    fn get(&self, key: Option<K>) -> Option<&V> {
        match key {
            Some(key) => self.some.get(key),
            None => self.none.as_ref(),
        }
    }

    #[inline]
    fn get_mut(&mut self, key: Option<K>) -> Option<&mut V> {
        match key {
            Some(key) => self.some.get_mut(key),
            None => self.none.as_mut(),
        }
    }

    #[inline]
    fn remove(&mut self, key: Option<K>) -> Option<V> {
        match key {
            Some(key) => self.some.remove(key),
            None => mem::replace(&mut self.none, None),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.some.clear();
        self.none = None;
    }

    #[inline]
    fn iter<'a>(&'a self, mut f: impl FnMut((Option<K>, &'a V))) {
        self.some.iter(|(k, v)| {
            f((Some(k), v));
        });

        if let Some(v) = self.none.as_ref() {
            f((None, v));
        }
    }

    #[inline]
    fn iter_mut<'a>(&'a mut self, mut f: impl FnMut((Option<K>, &'a mut V))) {
        self.some.iter_mut(|(k, v)| {
            f((Some(k), v));
        });

        if let Some(v) = self.none.as_mut() {
            f((None, v));
        }
    }
}