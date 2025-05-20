fn first_word(word: &str) -> &str {
    word.split_whitespace().next().unwrap_or("")
}

fn second_word(word: &str) -> &str {
    word.split_whitespace().nth(1).unwrap_or("")
}

fn main() {
    let test1 = String::from("hello beautiful world");
    println!("Test word: {}", test1);
    println!("First word: {}", first_word(&test1));
    println!("Second word: {}", second_word(&test1));

    let test2 = String::from("Rust");
    println!("Test word: {}", test2);
    println!("First word: {}", first_word(&test2));
    println!("Second word: {}", second_word(&test2));

    let test3 = String::from("is cool");
    println!("Test word: {}", test3);
    println!("First word: {}", first_word(&test3));
    println!("Second word: {}", second_word(&test3));

    let test4 = String::from("");
    println!("Test word: {}", test4);
    println!("First word: {}", first_word(&test4));
    println!("Second word: {}", second_word(&test4));

    let test5 = String::from(" first second ");
    println!("Test word: {}", test5);
    println!("First word: {}", first_word(&test5));
    println!("Second word: {}", second_word(&test5));
}
