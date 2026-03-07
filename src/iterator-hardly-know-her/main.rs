fn main() {
    // iter produces the iterator for map to iterate through, then collect collects the results of the map into a new vector
    // vector -> iterator via iter() --- iterator -> vector via collect()

    let data = vec![1, 2, 3]; // Cannot be passed into iter directly, see below:

    //vec![1,2,3].iter()`:
    // - `vec!` creates a temporary `Vec<i32>` (heap allocation, has `Drop`).
    // - `iter()` returns an iterator that borrows that `Vec`.
    // - The temporary `Vec` is dropped at the `;`, so the borrow would dangle.
    // - Rust rejects this (or effectively treats it as unusable beyond the statement).

    // `[1,2,3].iter()`:
    // - `[1,2,3]` is an array literal (no `Drop`, simple constant data).
    // - Rust can promote/extend this borrow context so the referenced data lives long enough.
    // - So the iterator is valid in your `let` binding and loop.

    let iter = data.iter();
    println!("{:?}", iter); // Iter([1, 2, 3])

    let map_iter = iter.map(|x| x + 1);
    println!("{:?}", map_iter); // Map { iter: Iter([1, 2, 3]) }

    // To this point the iterator has only been created and then transformed, but it has not been consumed. The map iterator is lazy, meaning it does not do any work until it is consumed. When we call collect(), it consumes the iterator and produces a new vector with the results of the map operation.
    let vec: Vec<_> = map_iter.collect();
    println!("{:?}", vec);

    // Iterator needs to be mutable in order to call next() on it, because next() changes the state of the iterator by advancing it to the next item.
    let mut myter = [1, 2, 3].iter().map(|x| x * x);
    let mut iter_copy = myter.clone(); // Clone the iterator to use it again later

    let mut new_vec: Vec<i32> = vec![];

    // Manually collect
    while let Some(x) = myter.next() {
        new_vec.push(x);
    }

    // The below is equivalent
    loop {
        match iter_copy.next() {
            Some(num) => new_vec.push(num),
            None => break,
        }
    }

    println!("{:?}", new_vec);

    let iter = [1, 2, 4, 5, 6, 3].iter();
    println!("{:?}", iter); // Iter([1, 2, 3, 4, 5, 3])

    let skip = iter.skip(2);
    println!("{:?}", skip); // Skip { iter: Iter([1, 2, 3, 4, 5, 3]), n: 2 }

    let taken = skip.take_while(|&&x| x > 3);
    println!("{:?}", taken); // TakeWhile { iter: Skip { iter: Iter([1, 2, 3, 4, 5, 3]), n: 2 }, flag: false }

    taken.for_each(|x| println!("{}", x)); // 4, 5
}

// Catamorphic operations in Rust, or catamorphisms, are generalized folds that consume recursive data structures to produce a single value, such as fold on iterators or map on Option/Result. They are used to replace explicit recursion, reducing complex structures to a simpler form or type, essentially acting as a structural "reduce"
