pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")

    let num_string = num.to_string();

    println!("Number string: {}", num_string);

    let num_length = num_string.chars().count() as u32;

    let mut armstrong: u64 = 0;

    for value in num_string.chars() {
        let digit = value.to_digit(10).expect("Not a digit") as u64;

        armstrong += digit.pow(num_length);
    }

    println!("Armstrong value: {}", armstrong);

    armstrong == num as u64
}

fn main() {
    is_armstrong_number(10);
}
