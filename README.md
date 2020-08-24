# Tupiter
This crate is a hacky solution for anyone who wants to iterate over a
homogeneous tuple that they can't rewrite to being an array.

```rs
use ::tupiter::IntoIterator;
for x in (1,2,3,4).into_iter() {
    println!("{}", x);
}
```
