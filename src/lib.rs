use std::mem::{transmute, MaybeUninit};

#[inline(never)]
pub fn naïve<I: IntoIterator<Item = u8>, const N: usize>(i: I) -> Box<[u8]> {
    let mut array = [0u8; N];
    array
        .iter_mut()
        .zip(i)
        .for_each(|(destination, source)| *destination = source);
    Box::new(array)
}

#[inline(never)]
pub fn maybe_uninit<I: IntoIterator<Item = u8>, const N: usize>(i: I) -> Box<[u8]> {
    let mut array: Box<[MaybeUninit<u8>; N]> = Box::new(
        // See
        // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        // for safety explanation.
        unsafe { MaybeUninit::uninit().assume_init() },
    );
    array.iter_mut().zip(i).for_each(|(destination, source)| {
        destination.write(source);
    });
    unsafe { transmute::<_, Box<[u8; N]>>(array) }
}
