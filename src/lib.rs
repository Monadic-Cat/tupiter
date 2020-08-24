//! Why the hell not iterate over homogeneously typed tuples?
//! Sure arrays are already gonna get better support for this,
//! but heck, there are libraries that give me tuples and I wanna map 'em!

pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}

macro_rules! maybe_uninit_uwu {
    ($type:ty | $($trash:tt)*) => { ::core::mem::MaybeUninit<$type> }
}
macro_rules! ty_uwu {
    ($type:ty | $($trash:tt)*) => { $type }
}

macro_rules! decl_tuple_iter_struct {
    ($name:ident | $($n:tt),*) => {
        pub struct $name<T> {
            tuple: ($( maybe_uninit_uwu!(T | $n) ,)*),
            idx: usize,
        }
        impl<T> ::core::iter::Iterator for $name<T> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                let cdx = self.idx;
                self.idx += 1;
                match cdx {
                    $(
                        $n => Some(unsafe { self.tuple.$n.as_mut_ptr().read() }),
                    )*
                    _ => None,
                }
            }
        }
        impl<T> $crate::IntoIterator for ($(ty_uwu!(T | $n),)*) {
            type Item = T;
            type IntoIter = $name<T>;
            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter {
                    tuple: ($(::core::mem::MaybeUninit::new(self.$n),)*),
                    idx: 0,
                }
            }
        }
    }
}
use ::tupiter_proc_macro::decl_for_range;
decl_for_range!(40);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
