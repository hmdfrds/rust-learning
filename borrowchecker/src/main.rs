fn inspect_string(content: &String) {
    if content.is_empty() {
        println!("String is empty.");
        return;
    }
    println!("String is not empty.");
    println!("The length of content is: {}", content.len());
}

fn add_empasis(text_to_modify: &mut String) {
    text_to_modify.push_str("!!!");
}

fn main() {
    let mut my_message = String::from("Important News");
    inspect_string(&my_message);
    add_empasis(&mut my_message);
    inspect_string(&my_message);

    // let another_message = &my_message;
    // add_empasis(&mut another_message);
}
