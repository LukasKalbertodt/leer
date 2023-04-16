//! Trait to abstract over types that have a notion of “being empty” and can
//! create such an empty instance. See [`Empty`]. Intended to be a foundational
//! crate.
//!
//!
//! ## Examples
//!
//! Using the trait:
//!
//! ```
//! use leer::Empty;
//!
//! let v = String::empty();
//! ```
//!
//! Mostly useful for something like this:
//!
//! ```
//! use leer::Empty;
//!
//! // Of course, in this specific case, `FromIterator` would be better. But
//! // you get the point.
//! fn one_two_three<C: Empty + Extend<u32>>() -> C {
//!     let mut out = C::empty();
//!     out.extend([1, 2, 3]);
//!     out
//! }
//!
//! let vec: Vec<_> = one_two_three();
//! let vec: std::collections::LinkedList<_> = one_two_three();
//! ```
//!
//! Deriving (with `derive` feature enabled):
//!
//! ```
//! use leer::Empty;
//!
//! #[derive(Empty)]
//! struct Zoo {
//!     foxes: Vec<Fox>,
//!     elephants: Vec<Elephant>,
//! }
//!
//! struct Fox;
//! struct Elephant;
//!
//!
//! let empty_zoo = Zoo::empty();
//! ```
//!
//!
//! ## Crate features
//!
//! - `derive`: if enabled, you can `#[derive(Empty)]` for structs.
//!

/// Types that have a notion of “being empty” and can create such an empty
/// instance.
///
/// This is very similar to `Default` from the standard library, but makes it
/// explicit that the returned instance is *empty* and not just any default
/// instance. This trait is implemented for several standard types.
pub trait Empty {
    /// Returns an empty value of this type.
    fn empty() -> Self;
}

/// Derive macro for [`Empty`].
///
/// This can only be derived for structs. All struct fields need to implement
/// `Empty` in order for the derive to work. If your struct has generic
/// parameters, they won't be bounded with `Empty` in the generated impl block.
/// This is useful most of the time, because things like `Vec<T>` and
/// `Option<T>` don't require `T: Empty` to implement `Empty`. But this means
/// that you sometimes have to add a global `Empty` bound to your parameter or
/// implement `Empty` manually.
///
/// ## Example
///
/// ```
/// use leer::Empty;
///
/// #[derive(Empty)]
/// struct MyStruct {
///     a: Vec<u32>,        // => `vec![]`
///     b: Option<String>,  // => `None`
///     c: (),              // => `()`
/// }
/// ```
#[cfg(feature = "derive")]
pub use leer_macros::Empty;


// ===========================================================================
// ===== Implementations
// ===========================================================================
macro_rules! impl_empty_via_default {
    ($( { $($impl_header:tt)+ } ,)*) => {
        $(
            $($impl_header)* {
                fn empty() -> Self {
                    Self::default()
                }
            }
        )*
    }
}

impl_empty_via_default!(
    { impl Empty for () },
    { impl<T: ?Sized> Empty for std::marker::PhantomData<T> },
    { impl<T> Empty for Option<T> },
    { impl Empty for String },
    { impl<T> Empty for Vec<T> },
    { impl<T: Ord> Empty for std::collections::BTreeSet<T> },
    { impl<T: Eq + std::hash::Hash> Empty for std::collections::HashSet<T> },
    { impl<T> Empty for std::collections::LinkedList<T> },
    { impl<T> Empty for std::collections::VecDeque<T> },
    { impl<K: Ord, V> Empty for std::collections::BTreeMap<K, V> },
    { impl<K: Eq + std::hash::Hash, V> Empty for std::collections::HashMap<K, V> },
);

impl<T: Empty> Empty for Box<T> {
    fn empty() -> Self {
        Box::new(T::empty())
    }
}

impl<A: Empty> Empty for (A,) {
    fn empty() -> Self { (A::empty(),) }
}
impl<A: Empty, B: Empty> Empty for (A, B) {
    fn empty() -> Self { (A::empty(), B::empty()) }
}
impl<A: Empty, B: Empty, C: Empty> Empty for (A, B, C) {
    fn empty() -> Self { (A::empty(), B::empty(), C::empty()) }
}
impl<A: Empty, B: Empty, C: Empty, D: Empty> Empty for (A, B, C, D) {
    fn empty() -> Self { (A::empty(), B::empty(), C::empty(), D::empty()) }
}

impl<T: Empty, const N: usize> Empty for [T; N] {
    fn empty() -> Self {
        std::array::from_fn(|_| T::empty())
    }
}
