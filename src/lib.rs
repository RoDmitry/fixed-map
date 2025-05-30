//! [<img alt="github" src="https://img.shields.io/badge/github-udoprog/fixed--map-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/fixed-map)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/fixed-map.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/fixed-map)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fixed--map-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/fixed-map)
//!
//! This crate provides a [`Map`] and [`Set`] container that can make use of a
//! pre-calculated backing storage. This enables the Rust compiler to heavily
//! optimize operations over them and avoid allocating.
//!
//! See [documentation] for information on how to use this crate.
//!
//! <br>
//!
//! ## Usage
//!
//! Add `fixed-map` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fixed-map = "0.9.5"
//! ```
//!
//! Anything used as a key in either a [`Map`] or a [`Set`] needs to implement
//! the [`Key`] trait. This should be derived:
//!
//! ```
//! use fixed_map::{Key, Map};
//!
//! #[derive(Clone, Copy, Key)]
//! enum MyKey {
//!     North,
//!     South,
//!     East,
//!     West,
//! }
//! ```
//!
//! After this you can use one of our containers:
//!
//! ```
//! # #[derive(Clone, Copy, fixed_map::Key)]
//! # enum MyKey { North, South, East, West }
//! use fixed_map::{Map, Set};
//!
//! let mut map = Map::new();
//! map.insert(MyKey::North, 200);
//! map.insert(MyKey::South, 100);
//!
//! assert_eq!(map.get(MyKey::North), Some(&200));
//! assert_eq!(map.get(MyKey::East), None);
//!
//! let mut set = Set::new();
//! set.insert(MyKey::North);
//! set.insert(MyKey::South);
//!
//! assert!(set.contains(MyKey::South));
//! assert!(!set.contains(MyKey::East));
//! ```
//!
//! <br>
//!
//! ## Features
//!
//! The following features are available:
//!
//! * `std` - Disabling this feature causes this crate to be no-std. This means
//!   that dynamic types cannot be used in keys, like ones enabled by the `map`
//!   feature (default).
//! * `hashbrown` - Causes [`Storage`] to be implemented by dynamic types such
//!   as `&'static str` or `u32`. These are backed by a `hashbrown` (default).
//! * `entry` - Enables an [`entry`] API similar to that found on [`HashMap`].
//! * `serde` - Causes [`Map`] and [`Set`] to implement [`Serialize`] and
//!   [`Deserialize`] if it's implemented by the key and value.
//!
//! <br>
//!
//! ## Specialized storage through the [`Key`] trait
//!
//! The [`Key` derive] is provided to instruct our containers on how to build
//! optimized storage for a given [`Key`]. We also require any key to be [`Copy`].
//!
//! ```
//! use fixed_map::Key;
//!
//! #[derive(Clone, Copy, Key)]
//! enum MyKey {
//!     North,
//!     South,
//!     East,
//!     West,
//! }
//! ```
//!
//! What happens behind the scenes is that a proc macro is used to build a
//! struct optimized for storing and indexing exactly 4 values - one for each
//! variant.
//!
//! Something exactly like this:
//!
//! ```no_run
//! struct Storage<V> {
//!     data: [Option<V>; 4],
//! }
//! ```
//!
//! It becomes a bit more complicated once we start considering *composite
//! keys*. See the [`Key`] documentation for more information.
//!
//! <br>
//!
//! ## Why does this crate exist?
//!
//! There are many cases where you want associate a value with a small, fixed
//! number of elements identified by an enum.
//!
//! Let's say you have a game where each room has something in four directions.
//! We can model this relationship between the direction and the item using two
//! enums.
//!
//! ```
//! #[repr(usize)]
//! pub enum Dir {
//!     North,
//!     East,
//!     South,
//!     West,
//! }
//!
//! pub enum Item {
//!     Bow,
//!     Sword,
//!     Axe,
//! }
//! ```
//!
//! The goal is for the performance of fixed map to be identical to storing the
//! data linearly in memory like you could through an array like `[Option<Item>;
//! N]` where each *index* corresponds to a variant in `Dir`.
//!
//! Doing this manually could look like this:
//!
//! ```
//! # #[repr(usize)]
//! # pub enum Dir { North, East, South, West }
//! # #[derive(Debug)]
//! # pub enum Item { Bow, Sword, Axe }
//! let mut map: [Option<Item>; 4] = [None, None, None, None];
//! map[Dir::North as usize] = Some(Item::Bow);
//!
//! if let Some(item) = &map[Dir::North as usize] {
//!     println!("found item: {:?}", item);
//! }
//! ```
//!
//! But with a fixed [`Map`] you can do it idiomatically like this, without
//! incurring a drop in performance:
//!
//! ```
//! use fixed_map::{Key, Map};
//!
//! #[derive(Clone, Copy, Key)]
//! pub enum Dir {
//!     North,
//!     East,
//!     South,
//!     West,
//! }
//!
//! #[derive(Debug)]
//! pub enum Item {
//!     Bow,
//!     Sword,
//!     Axe,
//! }
//!
//! let mut map = Map::new();
//! map.insert(Dir::North, Item::Bow);
//!
//! if let Some(item) = map.get(Dir::North) {
//!     println!("found item: {:?}", item);
//! }
//! ```
//!
//! <br>
//!
//! ## Unsafe use
//!
//! The Entry API uses `unwrap_unchecked` to obtain mutable references to the
//! inner value of `Some`s, and to skip `drop` when overwriting `None`s.
//!
//! <br>
//!
//! ## Benchmarks
//!
//! We include benchmarks to ensure that we abide by the expectation that a
//! fixed map or set should perform roughly the same as an array with the same
//! number of elements.
//!
//! In the following benchmarks fixed-map is compared to:
//!
//! * `fixed` - A [`Map`] with a derived [`Key`] with `N` variants.
//! * [`hashbrown`] - A high performance hash map. This is only included for
//!   reference.
//!   - Note: Maps are created with `HashMap::with_capacity(N)`.
//! * `array` - A simple `[Option<Key>; N]` array.
//!
//! Note: for all `insert` benchmarks the underlying storage is cloned in each
//! iteration.
//!
//! ```text
//! get/fixed/4             time:   [208.96 ps 209.57 ps 210.17 ps]
//! get/fixed/8             time:   [211.12 ps 211.86 ps 212.55 ps]
//! get/fixed/16            time:   [211.50 ps 211.84 ps 212.23 ps]
//! get/fixed/32            time:   [211.02 ps 211.40 ps 211.79 ps]
//! get/array/4             time:   [215.76 ps 216.56 ps 217.68 ps]
//! get/array/8             time:   [216.80 ps 217.28 ps 217.83 ps]
//! get/array/16            time:   [215.88 ps 216.21 ps 216.58 ps]
//! get/array/32            time:   [216.39 ps 216.82 ps 217.33 ps]
//! get/hashbrown/4         time:   [2.9134 ns 2.9168 ns 2.9210 ns]
//! get/hashbrown/8         time:   [2.9143 ns 2.9175 ns 2.9212 ns]
//! get/hashbrown/16        time:   [2.9258 ns 2.9293 ns 2.9328 ns]
//! get/hashbrown/32        time:   [2.9387 ns 2.9428 ns 2.9466 ns]
//!
//! insert/fixed/4          time:   [421.82 ps 422.47 ps 423.22 ps]
//! insert/fixed/8          time:   [635.46 ps 636.91 ps 638.55 ps]
//! insert/fixed/16         time:   [1.0579 ns 1.0599 ns 1.0621 ns]
//! insert/fixed/32         time:   [1.6991 ns 1.7016 ns 1.7043 ns]
//! insert/array/4          time:   [419.26 ps 419.76 ps 420.30 ps]
//! insert/array/8          time:   [624.30 ps 626.27 ps 628.33 ps]
//! insert/array/16         time:   [1.0444 ns 1.0467 ns 1.0490 ns]
//! insert/array/32         time:   [1.6828 ns 1.6904 ns 1.6990 ns]
//! insert/hashbrown/4      time:   [87.002 ns 87.233 ns 87.475 ns]
//! insert/hashbrown/8      time:   [96.995 ns 97.287 ns 97.589 ns]
//! insert/hashbrown/16     time:   [517.89 ns 518.66 ns 519.57 ns]
//! insert/hashbrown/32     time:   [156.10 ns 156.67 ns 157.30 ns]
//!
//! values/fixed/4          time:   [209.09 ps 209.51 ps 209.91 ps]
//! values/fixed/8          time:   [213.99 ps 215.34 ps 217.08 ps]
//! values/fixed/16         time:   [213.24 ps 213.94 ps 214.72 ps]
//! values/fixed/32         time:   [212.71 ps 213.82 ps 215.15 ps]
//! values/array/4          time:   [211.07 ps 211.78 ps 212.59 ps]
//! values/array/8          time:   [211.48 ps 212.03 ps 212.65 ps]
//! values/array/16         time:   [213.04 ps 213.49 ps 213.99 ps]
//! values/array/32         time:   [213.18 ps 213.78 ps 214.60 ps]
//! values/hashbrown/4      time:   [3.3965 ns 3.4007 ns 3.4056 ns]
//! values/hashbrown/8      time:   [3.8443 ns 3.8627 ns 3.8895 ns]
//! values/hashbrown/16     time:   [5.6312 ns 5.6666 ns 5.7029 ns]
//! values/hashbrown/32     time:   [8.7221 ns 8.7674 ns 8.8117 ns]
//!
//! array/sum_values        time:   [3.0394 ns 3.0463 ns 3.0534 ns]
//! fixed/sum_values        time:   [3.0503 ns 3.0559 ns 3.0619 ns]
//! ```
//!
//! <br>
//!
//! ## Examples
//!
//! Most examples are in place to test what kind of assembler they compile to.
//!
//! To do this, run:
//!
//! ```sh
//! RUSTFLAGS="--emit asm" cargo build --release --example <example>
//! ```
//!
//! You should be able to find the assembler generated in the target folder:
//!
//! ```sh
//! ls target/release/examples/
//! ```
//!
//! [`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
//! [`Deserialize`]: https://docs.rs/serde/1/serde/trait.Deserialize.html
//! [`hashbrown`]: https://github.com/Amanieu/hashbrown
//! [`Key` derive]: https://docs.rs/fixed-map/latest/fixed_map/derive.Key.html
//! [`Key`]: https://docs.rs/fixed-map/latest/fixed_map/derive.Key.html
//! [`Map`]: https://docs.rs/fixed-map/latest/fixed_map/map/struct.Map.html
//! [`entry`]: https://docs.rs/fixed-map/latest/fixed_map/map/struct.Map.html#method.entry
//! [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html#method.entry
//! [`Serialize`]: https://docs.rs/serde/1/serde/trait.Serialize.html
//! [`Set`]: https://docs.rs/fixed-map/latest/fixed_map/set/struct.Set.html
//! [`Storage`]: https://docs.rs/fixed-map/latest/fixed_map/storage/trait.Storage.html
//! [documentation]: https://docs.rs/fixed-map

#![no_std]
#![deny(missing_docs)]
#![allow(clippy::expl_impl_clone_on_copy)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::type_repetition_in_bounds)]

pub mod raw;

mod key;
pub use self::key::Key;

pub mod map;
#[doc(inline)]
pub use self::map::Map;

pub mod set;
#[doc(inline)]
pub use self::set::Set;

// Re-export the option bucket types for use in `derive(Key)`
#[doc(hidden)]
pub mod option_bucket;

#[doc(hidden)]
pub mod macro_support;

/// Derive to implement the [`Key`] trait.
///
/// This derive implements the [`Key`] trait for a given type.
///
/// The [`Key`] trait is what allows `fixed_map` to set up storage for a type
/// that will be the key in a fixed map.
///
/// > Note: this requires the `::fixed_map` crate to be in scope when it's
/// > derived.
///
/// <br>
///
/// ## Container attributes
///
/// <br>
///
/// Container attributes are attributes which apply directly onto container
/// types. For [`Key`] this is just enums.
///
/// <br>
///
/// #### `#[key(bitset)]`
///
/// This ensures that backing storage is performed with a bitset when used with
/// a [`Set`].
///
/// ```
/// use fixed_map::{Key, Set};
///
/// #[derive(Clone, Copy, Key)]
/// pub enum Regular {
///     First,
///     Second,
///     Third,
/// }
///
/// #[derive(Clone, Copy, Key)]
/// #[key(bitset)]
/// pub enum Bits {
///     First,
///     Second,
///     Third,
/// }
///
/// // Normal storage uses an array of booleans:
/// assert_eq!(core::mem::size_of::<Set<Regular>>(), 3);
///
/// // Bitset storage uses a single u8 (or other appropriate type based on size):
/// assert_eq!(core::mem::size_of::<Set<Bits>>(), 1);
/// ```
///
/// > **Note:** not all operations will be implemented when this attribute is
/// > present, so some container methods might not work.
///
/// <br>
///
/// ## Guide
///
/// Given the following enum:
///
/// ```
/// use fixed_map::Key;
///
/// #[derive(Clone, Copy, Key)]
/// pub enum MyKey {
///     First,
///     Second,
///     Third,
/// }
/// ```
///
/// It performs the following expansion:
///
/// ```ignore
/// use fixed_map::Key;
///
/// #[derive(Clone, Copy)]
/// pub enum MyKey {
///     First,
///     Second,
///     Third,
/// }
///
/// /// Build a storage struct containing an item for each key:
/// pub struct MyKeyMapStorage<V> {
///     /// Storage for `MyKey::First`.
///     f1: Option<V>,
///     /// Storage for `MyKey::Second`.
///     f2: Option<V>,
///     /// Storage for `MyKey::Third`.
///     f3: Option<V>,
/// }
///
/// /// Build a storage struct containing an element for each key:
/// pub struct MyKeySetStorage {
///     data: [bool; 3],
/// }
///
/// /// Implement map storage for key.
/// impl<V> ::fixed_map::map::MapStorage<MyKey, V> for MyKeyMapStorage<V> {
///     fn get(&self, key: MyKey) -> Option<&V> {
///         match key {
///             MyKey::First => self.f1.as_ref(),
///             MyKey::Second => self.f2.as_ref(),
///             MyKey::Third => self.f3.as_ref(),
///         }
///     }
///
///     /* skipped */
/// }
///
/// /// Implement set storage for key.
/// impl ::fixed_map::set::SetStorage<MyKey> for MyKeySetStorage {
///     fn contains(&self, key: MyKey) -> bool {
///         let [a, b, c] = &self.data;
///
///         match key {
///             MyKey::First => *a,
///             MyKey::Second => *b,
///             MyKey::Third => *c,
///         }
///     }
///
///     /* skipped */
/// }
///
/// impl<V> Default for MyKeyMapStorage<V> {
///     fn default() -> Self {
///         Self {
///             f1: None,
///             f2: None,
///             f3: None,
///         }
///     }
/// }
///
/// impl Default for MyKeySetStorage {
///     fn default() -> Self {
///         Self {
///             data: [false, false, false]
///         }
///     }
/// }
///
/// /// Implement the Key trait to point out storage.
/// impl ::fixed_map::Key for MyKey {
///     type MapStorage<V> = MyKeyMapStorage<V>;
///     type SetStorage = MyKeySetStorage;
/// }
/// ```
#[doc(inline)]
pub use fixed_map_derive::Key;
