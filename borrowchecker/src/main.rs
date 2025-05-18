fn inspect_string(content: &String) {
    if content.is_empty() {
        println!("String is empty.");
        return;
    }
    println!("String is not empty.");
    println!("The length of content is: {}", content.len());
}

fn main() {
    println!("Hello, world!");
}
