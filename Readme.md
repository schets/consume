This provides a consume ordering for when the compiler cannot elide the dependency

# Examples

```rust
extern crate consume;
use std::sync::atomic::{AtomicPtr, AtomicUsize};

fn consume_load(pt: &AtomicPtr<*const usize>) -> usize {
    // There's a data dependency on the loaded value, 
    // meaning that the compiler can't od silly things to
    // eliminate the dependency
    unsafe { *pt.load(Consume) }
}

fn incorrect_consume_load(ind: &AtomicUsize, vals: &Vec<usize>) -> usize {
    // There is no data dependency here
    // since the compiler can eliminate the loaded value
    // from the pointer index. Ensure that you don't have this.
    // Use consume at your own risk.
    let i = ind.load(consume::Consume);
    vals[ind - ind]
    // vals[ind] would probably be correct, unless some shenanigans with the
    // bounds checking occured. I only use vec for convenience, only
    // use consume if you understand all possible data paths taken by the compiler
}

```
