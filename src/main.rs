fn narcissistic(num: u64) -> bool {
    //  find number of digits in number
    let stringify_num = num.to_string();
    let len = stringify_num.len();

    // separate digits
    let num_chars = stringify_num.chars().collect::<Vec<char>>();

    // loop through
    let mut sum = 0;
    for c in num_chars {
        // convert char to digit
        let n: u64 = c.to_digit(10).unwrap().into();

        // expo by the len
        let e: u64 = n.pow(len as u32);

        // sum
        sum += e
    }

    sum == num
}

fn main() {
    dbg!(narcissistic(153));
}
