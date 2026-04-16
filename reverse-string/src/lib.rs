pub fn reverse(input: &str) -> String {
    // split input into an array of characters
    let mut input_chars: Vec<char> = input.chars().collect();

    // reverse array
    input_chars.reverse();

    // join reversed input characters to string
    let reversed_input: String = input_chars.iter().collect();

    println!("{:?}", reversed_input);

    return reversed_input;
}

fn main() {
    reverse("hey");
}
