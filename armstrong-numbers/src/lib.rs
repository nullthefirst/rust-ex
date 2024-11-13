pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;

    let mut sum = 0;
    for c in num_str.chars() {
        let digit = c.to_digit(10).unwrap(); // Safe unwrap as we know it's a digit
        sum += digit.pow(num_digits);
    }

    num == sum

    // let num_text = format!("{num}");

    // let num_digits = num_text.len();

    // let num_text_split: Vec<&str> = num_text.split("").collect();

    // let mut split_output_list: Vec<Result<u32, ParseIntError>> = vec![];

    // for item in num_text_split {
    //     let my_num: Result<u32, _> = item.parse();
    //     println!("{:?}", my_num);
    //     split_output_list.push(my_num);
    // }

    // let mut num_list: Vec<u32> = vec![];

    // for item in split_output_list {
    //     match item {
    //         Ok(num) => num_list.push(num),
    //         Err(err) => println!("{}", err),
    //     }
    // }

    // println!("{:?}", num_list);

    // let num_digits_u32: Result<u32, _> = num_digits.try_into();

    // let mut digits: Vec<u32> = vec![];

    // match num_digits_u32 {
    //     Ok(value) => digits.push(value),
    //     Err(_) => println!("usize too large for u32"),
    // }

    // println!("{:?}", digits);

    // let mut output: u32 = 0;

    // for item in num_list {
    //     output += item.pow(digits[0]);
    // }

    // println!("{}", output);

    // if num == output {
    //     true
    // } else {
    //     false
    // }
}
