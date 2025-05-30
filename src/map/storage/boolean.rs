// Iterators are confusing if they impl `Copy`.

#![allow(missing_copy_implementations)]

use core::iter;
use core::mem;
use core::option;

use crate::map::{Entry, MapStorage, OccupiedEntry, VacantEntry};
use crate::option_bucket::{NoneBucket, OptionBucket, SomeBucket};

const TRUE_BIT: u8 = 0b10;
const FALSE_BIT: u8 = 0b01;

type Iter<'a, V> = iter::Chain<
    iter::Map<option::Iter<'a, V>, fn(&'a V) -> (bool, &'a V)>,
    iter::Map<option::Iter<'a, V>, fn(&'a V) -> (bool, &'a V)>,
>;
type Values<'a, V> = iter::Chain<option::Iter<'a, V>, option::Iter<'a, V>>;
type IterMut<'a, V> = iter::Chain<
    iter::Map<option::IterMut<'a, V>, fn(&'a mut V) -> (bool, &'a mut V)>,
    iter::Map<option::IterMut<'a, V>, fn(&'a mut V) -> (bool, &'a mut V)>,
>;
type ValuesMut<'a, V> = iter::Chain<option::IterMut<'a, V>, option::IterMut<'a, V>>;
type IntoIter<V> = iter::Chain<
    iter::Map<option::IntoIter<V>, fn(V) -> (bool, V)>,
    iter::Map<option::IntoIter<V>, fn(V) -> (bool, V)>,
>;

/// [`MapStorage`] for [`bool`] types.
///
/// # Examples
///
/// ```
/// use fixed_map::{Key, Map};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Key)]
/// enum MyKey {
///     First(bool),
///     Second,
/// }
///
/// let mut a = Map::new();
/// a.insert(MyKey::First(false), 1);
///
/// assert_eq!(a.get(MyKey::First(true)), None);
/// assert_eq!(a.get(MyKey::First(false)), Some(&1));
/// assert_eq!(a.get(MyKey::Second), None);
///
/// assert!(a.iter().eq([(MyKey::First(false), &1)]));
/// assert!(a.values().copied().eq([1]));
/// assert!(a.keys().eq([MyKey::First(false)]));
/// ```
///
/// Iterator over boolean storage:
///
/// ```
/// use fixed_map::{Key, Map};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Key)]
/// enum MyKey {
///     Bool(bool),
///     Other,
/// }
///
/// let mut a = Map::new();
/// a.insert(MyKey::Bool(true), 1);
/// a.insert(MyKey::Bool(false), 2);
///
/// assert!(a.iter().eq([(MyKey::Bool(true), &1), (MyKey::Bool(false), &2)]));
/// assert_eq!(a.iter().rev().collect::<Vec<_>>(), vec![(MyKey::Bool(false), &2), (MyKey::Bool(true), &1)]);
/// ```

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BooleanMapStorage<V> {
    t: Option<V>,
    f: Option<V>,
}

/// See [`BooleanMapStorage::keys`].
pub struct Keys {
    bits: u8,
}

impl Clone for Keys {
    #[inline]
    fn clone(&self) -> Keys {
        Keys { bits: self.bits }
    }
}

impl Iterator for Keys {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits & TRUE_BIT != 0 {
            self.bits &= !TRUE_BIT;
            return Some(true);
        }

        if self.bits & FALSE_BIT != 0 {
            self.bits &= !FALSE_BIT;
            return Some(false);
        }

        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.bits.count_ones() as usize;
        (len, Some(len))
    }
}

impl DoubleEndedIterator for Keys {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.bits & FALSE_BIT != 0 {
            self.bits &= !FALSE_BIT;
            return Some(false);
        }

        if self.bits & TRUE_BIT != 0 {
            self.bits &= !TRUE_BIT;
            return Some(true);
        }

        None
    }
}

impl ExactSizeIterator for Keys {
    #[inline]
    fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }
}

pub struct Vacant<'a, V> {
    key: bool,
    inner: NoneBucket<'a, V>,
}

pub struct Occupied<'a, V> {
    key: bool,
    inner: SomeBucket<'a, V>,
}

impl<'a, V> VacantEntry<'a, bool, V> for Vacant<'a, V> {
    #[inline]
    fn key(&self) -> bool {
        self.key
    }

    #[inline]
    fn insert(self, value: V) -> &'a mut V {
        self.inner.insert(value)
    }
}

impl<'a, V> OccupiedEntry<'a, bool, V> for Occupied<'a, V> {
    #[inline]
    fn key(&self) -> bool {
        self.key
    }

    #[inline]
    fn get(&self) -> &V {
        self.inner.as_ref()
    }

    #[inline]
    fn get_mut(&mut self) -> &mut V {
        self.inner.as_mut()
    }

    #[inline]
    fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }

    #[inline]
    fn insert(&mut self, value: V) -> V {
        self.inner.replace(value)
    }

    #[inline]
    fn remove(self) -> V {
        self.inner.take()
    }
}

impl<V> MapStorage<bool, V> for BooleanMapStorage<V> {
    type Iter<'this>
        = Iter<'this, V>
    where
        V: 'this;
    type Keys<'this>
        = Keys
    where
        V: 'this;
    type Values<'this>
        = Values<'this, V>
    where
        V: 'this;
    type IterMut<'this>
        = IterMut<'this, V>
    where
        V: 'this;
    type ValuesMut<'this>
        = ValuesMut<'this, V>
    where
        V: 'this;
    type IntoIter = IntoIter<V>;
    type Occupied<'this>
        = Occupied<'this, V>
    where
        V: 'this;
    type Vacant<'this>
        = Vacant<'this, V>
    where
        V: 'this;

    #[inline]
    fn empty() -> Self {
        Self {
            t: Option::default(),
            f: Option::default(),
        }
    }

    #[inline]
    fn len(&self) -> usize {
        usize::from(self.t.is_some()).saturating_add(usize::from(self.f.is_some()))
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.t.is_none() && self.f.is_none()
    }

    #[inline]
    fn insert(&mut self, key: bool, value: V) -> Option<V> {
        if key {
            mem::replace(&mut self.t, Some(value))
        } else {
            mem::replace(&mut self.f, Some(value))
        }
    }

    #[inline]
    fn contains_key(&self, key: bool) -> bool {
        if key {
            self.t.is_some()
        } else {
            self.f.is_some()
        }
    }

    #[inline]
    fn get(&self, key: bool) -> Option<&V> {
        if key {
            self.t.as_ref()
        } else {
            self.f.as_ref()
        }
    }

    #[inline]
    fn get_mut(&mut self, key: bool) -> Option<&mut V> {
        if key {
            self.t.as_mut()
        } else {
            self.f.as_mut()
        }
    }

    #[inline]
    fn remove(&mut self, key: bool) -> Option<V> {
        if key {
            self.t.take()
        } else {
            self.f.take()
        }
    }

    #[inline]
    fn retain<F>(&mut self, mut func: F)
    where
        F: FnMut(bool, &mut V) -> bool,
    {
        if let Some(t) = self.t.as_mut() {
            if !func(true, t) {
                self.t = None;
            }
        }
        if let Some(f) = self.f.as_mut() {
            if !func(false, f) {
                self.f = None;
            }
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.t = None;
        self.f = None;
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        let map: fn(_) -> _ = |v| (true, v);
        let a = self.t.iter().map(map);
        let map: fn(_) -> _ = |v| (false, v);
        let b = self.f.iter().map(map);
        a.chain(b)
    }

    #[inline]
    fn keys(&self) -> Self::Keys<'_> {
        Keys {
            bits: if self.t.is_some() { TRUE_BIT } else { 0 }
                | if self.f.is_some() { FALSE_BIT } else { 0 },
        }
    }

    #[inline]
    fn values(&self) -> Self::Values<'_> {
        self.t.iter().chain(self.f.iter())
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let map: fn(_) -> _ = |v| (true, v);
        let a = self.t.iter_mut().map(map);
        let map: fn(_) -> _ = |v| (false, v);
        let b = self.f.iter_mut().map(map);
        a.chain(b)
    }

    #[inline]
    fn values_mut(&mut self) -> Self::ValuesMut<'_> {
        self.t.iter_mut().chain(self.f.iter_mut())
    }

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let map: fn(_) -> _ = |v| (true, v);
        let a = self.t.into_iter().map(map);
        let map: fn(_) -> _ = |v| (false, v);
        let b = self.f.into_iter().map(map);
        a.chain(b)
    }

    #[inline]
    fn entry(&mut self, key: bool) -> Entry<'_, Self, bool, V> {
        if key {
            match OptionBucket::new(&mut self.t) {
                OptionBucket::Some(inner) => Entry::Occupied(Occupied { key, inner }),
                OptionBucket::None(inner) => Entry::Vacant(Vacant { key, inner }),
            }
        } else {
            match OptionBucket::new(&mut self.f) {
                OptionBucket::Some(inner) => Entry::Occupied(Occupied { key, inner }),
                OptionBucket::None(inner) => Entry::Vacant(Vacant { key, inner }),
            }
        }
    }
}
