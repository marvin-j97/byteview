//! An immutable byte slice that may be inlined, and can be partially cloned without heap allocation.
//!
//! ```
//! # use byteview::ByteView;
//! let slice = ByteView::from("helloworld_thisisaverylongstring");
//!
//! // No heap allocation - increases the ref count like an Arc<[u8]>
//! let full_copy = slice.clone();
//! drop(full_copy);
//!
//! // No heap allocation - increases the ref count like an Arc<[u8]>, but we only get a subslice
//! let copy = slice.slice(11..);
//! assert_eq!(b"thisisaverylongstring", &*copy);
//!
//! // No heap allocation - if the slice is small enough, it will be inlined into the struct...
//! let copycopy = copy.slice(0..4);
//! assert_eq!(b"this", &*copycopy);
//!
//! // ...so no ref count incrementing is done
//! assert_eq!(2, slice.ref_count());
//!
//! drop(copy);
//! assert_eq!(1, slice.ref_count());
//!
//! drop(copycopy);
//! assert_eq!(1, slice.ref_count());
//!
//! // Our original slice will be automatically freed if all slices vanish
//! drop(slice);
//! ```

use std::{
    ops::Deref,
    sync::Arc,
    sync::{atomic::AtomicU64, atomic::Ordering},
};

#[cfg(target_pointer_width = "64")]
const INLINE_SIZE: usize = 20;

#[cfg(target_pointer_width = "32")]
const INLINE_SIZE: usize = 16;

const PREFIX_SIZE: usize = 4;

#[repr(C)]
struct HeapAllocationHeader {
    ref_count: AtomicU64,
}

// TODO: track allocations somehow in tests

/// An immutable byte slice
///
/// Will be inlined (no pointer dereference or heap allocation)
/// if it is 20 characters or shorter (on a 64-bit system).
///
/// A single heap allocation will be shared between multiple slices.
/// Even subslices of that heap allocation can be cloned without additional heap allocation.
///
/// The design is very similar to:
///
/// - [Polars' strings](<https://pola.rs/posts/polars-string-type>)
/// - [CedarDB's German strings](<https://cedardb.com/blog/german_strings>)
/// - [Umbra's string](<https://db.in.tum.de/~freitag/papers/p29-neumann-cidr20.pdf>)
/// - [Velox' String View](https://facebookincubator.github.io/velox/develop/vectors.html)
/// - [Apache Arrow's String View](https://arrow.apache.org/docs/cpp/api/datatype.html#_CPPv4N5arrow14BinaryViewType6c_typeE)
#[repr(C)]
pub struct ByteView {
    len: u32,
    prefix: [u8; PREFIX_SIZE],
    rest: [u8; 8],
    data: *const u8,
}

impl Default for ByteView {
    fn default() -> Self {
        Self {
            len: 0,
            prefix: [0; 4],
            rest: [0; 8],
            data: std::ptr::null(),
        }
    }
}

impl Clone for ByteView {
    fn clone(&self) -> Self {
        self.slice(..)
    }
}

impl Drop for ByteView {
    fn drop(&mut self) {
        if self.is_inline() {
            return;
        }

        let heap_region = self.get_heap_region();

        if heap_region.ref_count.fetch_sub(1, Ordering::AcqRel) != 1 {
            return;
        }

        unsafe {
            // The RC is now 0, so free heap allocation
            let ptr = heap_region as *const HeapAllocationHeader as *mut u8;
            drop(Box::from_raw(ptr));
        }
    }
}

impl Eq for ByteView {}

impl std::cmp::PartialEq for ByteView {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let src_ptr = self as *const ByteView as *const u8;
            let other_ptr: *const u8 = other as *const ByteView as *const u8;

            let a = *(src_ptr as *const u64);
            let b = *(other_ptr as *const u64);

            if a != b {
                return false;
            }
        }

        // NOTE: At this point we know
        // both strings must have the same prefix and same length
        //
        // If we are inlined, the other string must be inlined too,
        // so checking the prefix is enough
        if self.is_inline() {
            return self.rest == other.rest;
        }

        let len = self.len as usize;
        self.get_long_slice(len) == other.get_long_slice(len)
    }
}

impl std::cmp::Ord for ByteView {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::Equal;

        self.prefix.cmp(&other.prefix).then_with(|| {
            if self.len <= 4 && other.len <= 4 {
                self.len.cmp(&other.len)
            } else if self.is_inline() && other.is_inline() {
                match self.rest.cmp(&other.rest) {
                    Equal => self.len.cmp(&other.len),
                    x => x,
                }
            } else {
                self.deref().cmp(other.deref())
            }
        })
    }
}

impl std::cmp::PartialOrd for ByteView {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for ByteView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Deref for ByteView {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let len = self.len as usize;

        if self.is_inline() {
            self.get_short_slice(len)
        } else {
            self.get_long_slice(len)
        }
    }
}

impl std::hash::Hash for ByteView {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl ByteView {
    #[inline]
    const fn is_inline(&self) -> bool {
        self.len <= INLINE_SIZE as u32
    }

    /// Creates a new slice from an existing byte slice.
    ///
    /// Will heap-allocate the slice if it has at least length 13.
    pub fn new(slice: &[u8]) -> Self {
        let slice_len = slice.len();

        let Ok(len) = u32::try_from(slice_len) else {
            panic!("byte slice too long");
        };

        let mut str = Self {
            len,
            data: std::ptr::null(),
            prefix: [0; PREFIX_SIZE],
            rest: [0; 8],
        };

        if str.is_inline() {
            unsafe {
                // SAFETY: We check for inlinability
                // so we know the the input slice fits our buffer
                std::ptr::copy_nonoverlapping(slice.as_ptr(), str.prefix.as_mut_ptr(), slice_len)
            }
        } else {
            str.prefix.copy_from_slice(&slice[0..PREFIX_SIZE]);

            unsafe {
                // Heap allocation, with exactly enough bytes for the header + slice length
                let layout = std::alloc::Layout::array::<u8>(
                    std::mem::size_of::<HeapAllocationHeader>() + slice_len,
                )
                .unwrap();

                let heap_ptr = std::alloc::alloc(layout);

                // SAFETY: We store a pointer to the copied slice, which comes directly after the header
                str.data = heap_ptr.add(std::mem::size_of::<HeapAllocationHeader>());

                // Copy byte slice into heap allocation
                std::ptr::copy_nonoverlapping(slice.as_ptr(), str.data as *mut u8, slice_len);

                // Set pointer in "rest" to heap allocation address
                let ptr = heap_ptr as u64;
                let ptr_bytes = ptr.to_le_bytes();
                str.rest = ptr_bytes;

                // Set ref_count to 1
                let ref_count = heap_ptr as *mut u64;
                *ref_count = 1;
            }
        }

        debug_assert_eq!(slice, &*str);
        debug_assert_eq!(1, str.ref_count());
        debug_assert_eq!(str.len, slice.len() as u32);

        str
    }

    fn get_heap_region(&self) -> &HeapAllocationHeader {
        debug_assert!(
            !self.is_inline(),
            "inline slice does not have a heap allocation"
        );

        unsafe {
            // SAFETY: Shall only be used when the slice is not inlined
            // otherwise the heap pointer would be garbage
            let ptr = u64::from_le_bytes(self.rest);
            let ptr = ptr as *const u8;

            let heap_alloc_region: *const HeapAllocationHeader = ptr as *const HeapAllocationHeader;
            &*heap_alloc_region
        }
    }

    /// Returns the ref_count of the underlying heap allocation.
    #[doc(hidden)]
    pub fn ref_count(&self) -> u64 {
        if self.is_inline() {
            1
        } else {
            self.get_heap_region().ref_count.load(Ordering::Acquire)
        }
    }

    /// Clones the contents of this slice into a vector.
    pub fn to_vec(&self) -> Vec<u8> {
        self.deref().to_vec()
    }

    // TODO: should return ByteView with its own RC
    /*  /// Clones the contents of this slice into an independently tracked slice.
    pub fn to_owned(&self) -> Arc<[u8]> {
        self.deref().into()
    } */

    /// Clones the given range of the existing slice without heap allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use byteview::ByteView;
    /// let slice = ByteView::from("helloworld_thisisalongstring");
    /// let copy = slice.slice(11..);
    /// assert_eq!(b"thisisalongstring", &*copy);
    /// ```
    pub fn slice(&self, range: impl std::ops::RangeBounds<usize>) -> Self {
        use core::ops::Bound;

        // Credits: This is essentially taken from
        // https://github.com/tokio-rs/bytes/blob/291df5acc94b82a48765e67eeb1c1a2074539e68/src/bytes.rs#L264

        let self_len = self.len();

        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n.checked_add(1).expect("out of range"),
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self_len,
        };

        assert!(
            begin <= end,
            "range start must not be greater than end: {:?} <= {:?}",
            begin,
            end,
        );
        assert!(
            end <= self_len,
            "range end out of bounds: {:?} <= {:?}",
            end,
            self_len,
        );

        let new_len = end - begin;

        // Target and destination slices are inlined
        // so we just need to memcpy the struct, and replace
        // the inline slice with the requested range
        if new_len <= INLINE_SIZE && self_len <= INLINE_SIZE {
            let mut cloned = Self {
                len: u32::try_from(new_len).unwrap(),
                data: std::ptr::null(),
                prefix: [0; PREFIX_SIZE],
                rest: [0; 8],
            };

            let slice = &self.get_short_slice(self.len())[begin..end];
            debug_assert_eq!(slice.len(), new_len);

            unsafe {
                std::ptr::copy_nonoverlapping(slice.as_ptr(), cloned.prefix.as_mut_ptr(), new_len);
            }

            cloned
        } else if new_len <= INLINE_SIZE && self_len > INLINE_SIZE {
            let mut cloned = Self {
                len: u32::try_from(new_len).unwrap(),
                data: std::ptr::null(),
                prefix: [0; PREFIX_SIZE],
                rest: [0; 8],
            };

            let slice = &self.get_long_slice(self.len())[begin..end];
            debug_assert_eq!(slice.len(), new_len);

            unsafe {
                std::ptr::copy_nonoverlapping(slice.as_ptr(), cloned.prefix.as_mut_ptr(), new_len);
            }

            cloned
        } else if new_len > INLINE_SIZE && self_len > INLINE_SIZE {
            let heap_region = self.get_heap_region();
            let rc_before = heap_region.ref_count.fetch_add(1, Ordering::Release);
            debug_assert!(rc_before < u64::MAX, "refcount overflow");

            let mut cloned = Self {
                len: u32::try_from(new_len).unwrap(),
                // SAFETY: self.data must be defined
                // we cannot get a range larger than our own slice
                // so we cannot be inlined while the requested slice is not inlinable
                data: unsafe { self.data.add(begin) },
                prefix: [0; PREFIX_SIZE],
                rest: self.rest,
            };

            let prefix = &self.get_long_slice(self.len())[begin..(begin + 4)];
            debug_assert_eq!(prefix.len(), 4);
            cloned.prefix.copy_from_slice(prefix);

            cloned
        } else {
            unreachable!()
        }
    }

    /// Returns `true` if the slice is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the amount of bytes in the slice.
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    fn get_short_slice(&self, len: usize) -> &[u8] {
        debug_assert!(
            len <= INLINE_SIZE,
            "cannot get short slice - slice is not inlined"
        );

        // SAFETY: Shall only be called if slice is inlined
        unsafe { std::slice::from_raw_parts(self.prefix.as_ptr(), len) }
    }

    fn get_long_slice(&self, len: usize) -> &[u8] {
        debug_assert!(
            len > INLINE_SIZE,
            "cannot get long slice - slice is inlined"
        );

        // SAFETY: Shall only be called if slice is heap allocated
        unsafe { std::slice::from_raw_parts(self.data, len) }
    }
}

impl AsRef<[u8]> for ByteView {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl FromIterator<u8> for ByteView {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        Self::from(iter.into_iter().collect::<Vec<u8>>())
    }
}

impl From<&[u8]> for ByteView {
    fn from(value: &[u8]) -> Self {
        Self::new(value)
    }
}

impl From<Arc<[u8]>> for ByteView {
    fn from(value: Arc<[u8]>) -> Self {
        Self::new(&value)
    }
}

impl From<Vec<u8>> for ByteView {
    fn from(value: Vec<u8>) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for ByteView {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<String> for ByteView {
    fn from(value: String) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<Arc<str>> for ByteView {
    fn from(value: Arc<str>) -> Self {
        Self::from(&*value)
    }
}

impl<const N: usize> From<[u8; N]> for ByteView {
    fn from(value: [u8; N]) -> Self {
        Self::from(value.as_slice())
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::ByteView;
    use serde::de::{self, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;
    use std::ops::Deref;

    impl Serialize for ByteView {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_bytes(self.deref())
        }
    }

    impl<'de> Deserialize<'de> for ByteView {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct ByteViewVisitor;

            impl<'de> Visitor<'de> for ByteViewVisitor {
                type Value = ByteView;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a byte array")
                }

                fn visit_bytes<E>(self, v: &[u8]) -> Result<ByteView, E>
                where
                    E: de::Error,
                {
                    Ok(ByteView::from(v))
                }
            }

            deserializer.deserialize_bytes(ByteViewVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ByteView, HeapAllocationHeader};

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn memsize() {
        assert_eq!(24, std::mem::size_of::<ByteView>());
        assert_eq!(
            32,
            std::mem::size_of::<ByteView>() + std::mem::size_of::<HeapAllocationHeader>()
        );
    }

    #[test]
    fn cmp_misc_1() {
        let a = ByteView::from("abcdef");
        let b = ByteView::from("abcdefhelloworldhelloworld");
        assert!(a < b);
    }

    #[test]
    fn nostr() {
        let slice = ByteView::from("");
        assert_eq!(0, slice.len());
        assert_eq!(&*slice, b"");
        assert_eq!(1, slice.ref_count());
        assert!(slice.is_inline());
    }

    #[test]
    fn default_str() {
        let slice = ByteView::default();
        assert_eq!(0, slice.len());
        assert_eq!(&*slice, b"");
        assert_eq!(1, slice.ref_count());
        assert!(slice.is_inline());
    }

    #[test]
    fn short_str() {
        let slice = ByteView::from("abcdef");
        assert_eq!(6, slice.len());
        assert_eq!(&*slice, b"abcdef");
        assert_eq!(1, slice.ref_count());
        assert_eq!(&slice.prefix, b"abcd");
        assert!(slice.is_inline());
    }

    #[test]
    fn medium_str() {
        let slice = ByteView::from("abcdefabcdef");
        assert_eq!(12, slice.len());
        assert_eq!(&*slice, b"abcdefabcdef");
        assert_eq!(1, slice.ref_count());
        assert_eq!(&slice.prefix, b"abcd");
        assert!(slice.is_inline());
    }

    #[test]
    fn medium_long_str() {
        let slice = ByteView::from("abcdefabcdefabcdabcd");
        assert_eq!(20, slice.len());
        assert_eq!(&*slice, b"abcdefabcdefabcdabcd");
        assert_eq!(1, slice.ref_count());
        assert_eq!(&slice.prefix, b"abcd");
        assert!(slice.is_inline());
    }

    #[test]
    fn medium_str_clone() {
        let slice = ByteView::from("abcdefabcdefabcdefab");
        let copy = slice.clone();
        assert_eq!(slice, copy);
        assert_eq!(copy.prefix, slice.prefix);

        assert_eq!(1, slice.ref_count());

        drop(copy);
        assert_eq!(1, slice.ref_count());
    }

    #[test]
    fn long_str() {
        let slice = ByteView::from("abcdefabcdefabcdefababcd");
        assert_eq!(24, slice.len());
        assert_eq!(&*slice, b"abcdefabcdefabcdefababcd");
        assert_eq!(1, slice.ref_count());
        assert_eq!(&slice.prefix, b"abcd");
        assert!(!slice.is_inline());
    }

    #[test]
    fn long_str_clone() {
        let slice = ByteView::from("abcdefabcdefabcdefababcd");
        let copy = slice.clone();
        assert_eq!(slice, copy);
        assert_eq!(copy.prefix, slice.prefix);

        assert_eq!(2, slice.ref_count());

        drop(copy);
        assert_eq!(1, slice.ref_count());
    }

    #[test]
    fn long_str_slice_full() {
        let slice = ByteView::from("helloworld_thisisalongstring");

        let copy = slice.slice(..);
        assert_eq!(copy, slice);

        assert_eq!(2, slice.ref_count());

        drop(copy);
        assert_eq!(1, slice.ref_count());
    }

    #[test]
    fn long_str_slice() {
        let slice = ByteView::from("helloworld_thisisalongstring");

        let copy = slice.slice(11..);
        assert_eq!(b"thisisalongstring", &*copy);
        assert_eq!(&copy.prefix, b"this");

        assert_eq!(1, slice.ref_count());

        drop(copy);
        assert_eq!(1, slice.ref_count());
    }

    #[test]
    fn long_str_slice_twice() {
        let slice = ByteView::from("helloworld_thisisalongstring");

        let copy = slice.slice(11..);
        assert_eq!(b"thisisalongstring", &*copy);

        let copycopy = copy.slice(..);
        assert_eq!(copy, copycopy);

        assert_eq!(1, slice.ref_count());

        drop(copy);
        assert_eq!(1, slice.ref_count());

        drop(slice);
        assert_eq!(1, copycopy.ref_count());
    }

    #[test]
    fn long_str_slice_downgrade() {
        let slice = ByteView::from("helloworld_thisisalongstring");

        let copy = slice.slice(11..);
        assert_eq!(b"thisisalongstring", &*copy);

        let copycopy = copy.slice(0..4);
        assert_eq!(b"this", &*copycopy);

        assert_eq!(1, slice.ref_count());

        drop(copy);
        assert_eq!(1, slice.ref_count());

        drop(copycopy);
        assert_eq!(1, slice.ref_count());
    }

    #[test]
    fn short_str_clone() {
        let slice = ByteView::from("abcdef");
        let copy = slice.clone();
        assert_eq!(slice, copy);

        assert_eq!(1, slice.ref_count());

        drop(slice);
        assert_eq!(&*copy, b"abcdef");

        assert_eq!(1, copy.ref_count());
    }

    #[test]
    fn short_str_slice_full() {
        let slice = ByteView::from("abcdef");
        let copy = slice.slice(..);
        assert_eq!(slice, copy);

        assert_eq!(1, slice.ref_count());

        drop(slice);
        assert_eq!(&*copy, b"abcdef");

        assert_eq!(1, copy.ref_count());
    }

    #[test]
    fn short_str_slice_part() {
        let slice = ByteView::from("abcdef");
        let copy = slice.slice(3..);

        assert_eq!(1, slice.ref_count());

        drop(slice);
        assert_eq!(&*copy, b"def");

        assert_eq!(1, copy.ref_count());
    }

    #[test]
    fn short_str_slice_empty() {
        let slice = ByteView::from("abcdef");
        let copy = slice.slice(0..0);

        assert_eq!(1, slice.ref_count());

        drop(slice);
        assert_eq!(&*copy, b"");

        assert_eq!(1, copy.ref_count());
    }

    #[test]
    fn tiny_str_cmp() {
        let a = ByteView::from("abc");
        let b = ByteView::from("def");
        assert!(a < b);
    }

    #[test]
    fn tiny_str_eq() {
        let a = ByteView::from("abc");
        let b = ByteView::from("def");
        assert!(a != b);
    }

    #[test]
    fn long_str_eq() {
        let a = ByteView::from("abcdefabcdefabcdefabcdef");
        let b = ByteView::from("xycdefabcdefabcdefabcdef");
        assert!(a != b);
    }

    #[test]
    fn long_str_cmp() {
        let a = ByteView::from("abcdefabcdefabcdefabcdef");
        let b = ByteView::from("xycdefabcdefabcdefabcdef");
        assert!(a < b);
    }

    #[test]
    fn long_str_eq_2() {
        let a = ByteView::from("abcdefabcdefabcdefabcdef");
        let b = ByteView::from("abcdefabcdefabcdefabcdef");
        assert!(a == b);
    }

    #[test]
    fn long_str_cmp_2() {
        let a = ByteView::from("abcdefabcdefabcdefabcdef");
        let b = ByteView::from("abcdefabcdefabcdefabcdeg");
        assert!(a < b);
    }

    #[test]
    fn long_str_cmp_3() {
        let a = ByteView::from("abcdefabcdefabcdefabcde");
        let b = ByteView::from("abcdefabcdefabcdefabcdef");
        assert!(a < b);
    }

    #[test]
    fn cmp_fuzz_1() {
        let a = ByteView::from([0]);
        let b = ByteView::from([]);

        assert!(a > b);
    }

    #[test]
    fn cmp_fuzz_2() {
        let a = ByteView::from([0, 0]);
        let b = ByteView::from([0]);

        assert!(a > b);
    }

    #[test]
    fn cmp_fuzz_3() {
        let a = ByteView::from([255, 255, 12, 255, 0]);
        let b = ByteView::from([255, 255, 12, 255]);

        assert!(a > b);
    }
}
