# OrdBy

Conveniently attach a custom compare function to any type, to get
[sort_by()](std::vec::Vec#method.sort_by) type behavior from types like [BinaryHeap](std::collections::BinaryHeap)

OrdBy is a generalization of the pattern used by [Reverse].

Conveniently attach a custom compare function to any type, to get
[sort_by](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by) type behavior from types like [BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)


```rust
use std::collections::BinaryHeap;
use ord_by::*;

let numbers = vec![0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
let mut heap = BinaryHeap::with_capacity(10);

numbers.into_iter().ord_by(|a, b| a.cmp(b))
    .for_each(|n| heap.push(n));

let mut sorted = Vec::with_capacity(10);
while let Some(n) = heap.pop() {
    sorted.push(n.into_inner())
}

assert_eq!(sorted, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
```

**Future Work:** I would like the compare function to be a single associated constant for
efficiency, rather than being a variable stored with each struct instance.  However that
requires associated constants that can have a type dependent on generics, which is currently
an unsupported feature.
<https://github.com/rust-lang/rust/issues/98210>
