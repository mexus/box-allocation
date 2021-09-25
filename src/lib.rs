use std::{
    alloc::Layout,
    mem::{transmute, MaybeUninit},
};

/// A "naïve" approach: an array on the stack is created, then filled and boxed.
#[inline(never)]
pub fn naïve<I: IntoIterator<Item = u8>, const N: usize>(i: I) -> Box<[u8]> {
    let mut array = [0u8; N];
    array
        .iter_mut()
        .zip(i)
        .for_each(|(destination, source)| *destination = source);
    Box::new(array)
}

/// A "maybe-uninit" approach: an "uninitialized" box is created from an
/// allocation, then filled.
#[inline(never)]
pub fn maybe_uninit<I: IntoIterator<Item = u8>, const N: usize>(i: I) -> Box<[u8]> {
    let mut array: Box<[MaybeUninit<u8>; N]> = {
        let begin = if N == 0 {
            Box::into_raw(Box::new([] as [u8; 0])).cast::<u8>()
        } else {
            unsafe {
                // Safety: it's safe to allocate data of a non-zero size.
                std::alloc::alloc(Layout::new::<[u8; N]>())
            }
        };
        unsafe {
            // Safety: so long as T: Sized, a Box<T> is guaranteed to be represented
            // as a single pointer, see
            // https://doc.rust-lang.org/std/boxed/index.html#memory-layout
            Box::from_raw(begin.cast())
        }
    };
    array.iter_mut().zip(i).for_each(|(destination, source)| {
        destination.write(source);
    });
    unsafe { 
        // Safety: Box<[MaybeUninit<u8>; N]> and Box<[u8; N]> have the same
        // layout.
        transmute::<_, Box<[u8; N]>>(array) }
}
