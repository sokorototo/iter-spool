## IterSpool (written as practice extending Rust's internal API's)

Adds an extra `.spool` method to Iterators that allows one to consume an
iterator until a condition is met, similar to how `fold` works but with
termination. The last calculated accumulation is returned as the result of the
spool. The `Fuse` enum is used to specify a termination. `Fuse::Continue`
continues the fold, while `Fuse::Break` terminates it and yields it's value.

> This can be used to prevent consuming a whole iterator while checking a predicate, borrowing features from `find` and `fold`.

# Example Usage

```rust
// Count if there are more than 3 trues in an iterator, Conway's GoL style
use crate::{Fuse, Spool};

#[test]
fn it_works() {
   let neighbors = [true, true, true, true, true, false, true, false];
   let maximum_cells = 3;

   let number_of_true = neighbors
      .iter()
      .spool(0, |acc, condition| {
            if *condition {
               if acc < maximum_cells {
                  Fuse::Continue(acc + 1)
               } else {
                  Fuse::Break(acc)
               }
            } else {
               Fuse::Continue(acc)
            }
      });

   dbg!(number_of_true > maximum_cells);
}
```
