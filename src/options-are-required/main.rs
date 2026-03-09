fn practice(num: Option<isize>) -> isize {
    num.unwrap_or(0) * 5
}

// Now we want to return None if None
fn practice2(num: Option<isize>) -> Option<isize> {
    // Map will only run the lambda if num is Some, otherwise it will return None
    num.map(|x| x * 5)
}

fn practice3(num: Option<isize>) -> Option<isize> {
    let num_val = num?;

    // Effectively under the hood the "?" operator is doing a match that returns Some(num) or None, like below
    let val = match num {
        Some(val) => val,
        None => return None,
    };

    println!("Val: {}", val);
    println!("Num: {}", num_val);
    println!("Num equals Val: {}", num_val == val);

    return Some(num_val * 5);
}

fn practice4(num: Option<isize>) -> Option<isize> {
    // The end result of practice4
    Some(num? * 5)
}

fn practice5(nums: &Vec<i32>, index: usize) -> i32 {
    nums.get(index).unwrap_or(&(index as i32)) * 5
}

fn main() {
    println!("Result: {}", practice(Option::Some(-10)));
    println!("Result: {}\n", practice(Option::None));

    println!("Result: {:?}", practice2(Some(-10)));
    println!("Result: {:?}\n", practice2(None));

    println!("Result: {:?}\n", practice3(Some(-10)));
    println!("Result: {:?}\n", practice3(None));

    println!("Result: {:?}", practice4(Some(-10)));
    println!("Result: {:?}\n", practice4(None));

    let nums = vec![1, 2, -3];

    println!("Result: {:?}", practice5(&nums, 2)); // nums[2] exists, 3 * 5 == 15
    println!("Result: {:?}\n", practice5(&nums, 10)); // nums[10] is None, 3 * 10 == 50
}
