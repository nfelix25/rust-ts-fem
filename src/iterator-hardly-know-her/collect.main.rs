use std::collections::HashSet;

// collect() will automagically use the type of the variable it's being assigned to, so we don't need to specify it here.
//
// Example: The type of sentence is String, so collect() will know to collect into a String.

fn main() {
    let data = vec!["This", "is", "a", "sentence", "."];
    let length = data.len();

    let sentence: String = data
        .into_iter()
        .enumerate() // .enumerate() gives us (index, value)
        .map(|(i, s)| {
            if i < length - 2 {
                format!("{} ", s)
            } else {
                format!("{}", s)
            }
        })
        .collect();

    println!("{}", sentence);

    // Below since the type of my_set is HashSet<isize>, collect() will know to collect into a HashSet<isize>. The duplicate 1 will be removed because HashSet only keeps unique values.

    let my_set: HashSet<isize> = vec![1, 2, 3, 1].into_iter().collect();

    println!("{:?}", my_set);
}
