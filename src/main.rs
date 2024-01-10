fn sort_the_odd(arr: &[i32]) -> Vec<i32> {
    // detect odd and even numbers
    fn is_even(num: &i32) -> bool {
        num % 2 == 0
    }

    // separate odd and even numbers
    let mut odds = Vec::new();
    let mut evens = Vec::new();
    for (i, num) in arr.iter().enumerate() {
        match is_even(num) {
            true => evens.push((i, num)), // save index key for evens
            false => odds.push(*num),
        }
    }

    // sort odds
    odds.sort();

    // insert the evens
    for even in evens {
        odds.insert(even.0, *even.1);
    }

    odds
}

fn main() {
    dbg!(sort_the_odd(&[5, 8, 6, 3, 4]));
}
