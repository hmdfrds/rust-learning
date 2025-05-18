fn process_string(mut input_str: String) -> String {
    input_str.push_str(" processed");
    input_str
}
fn calculate_length_and_give_back(data: String) -> (String, usize) {
    let len = data.len();
    (data, len)
}

fn main() {
    let original = String::from("hello");
    let processed_original = process_string(original);
    println!("{}", processed_original);

    let another_message = String::from("Rust is shet");
    let (returned_message, len) = calculate_length_and_give_back(another_message);
    println!("The len of {} is {}", returned_message, len);
}
