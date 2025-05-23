struct TextContext<'a> {
    text: &'a str,
}

impl<'a> TextContext<'a> {
    fn get_first_sentence(&self) -> &'a str {
        if let Some(idx) = self.text.find(['.', '?', '!']) {
            return &self.text[..idx + 1];
        }
        self.text
    }
}

fn get_shared_prefix<'a>(text1: &'a str, text2: &'a str) -> &'a str {
    let mut prefix_byte_len = 0;
    for (char1, char2) in text1.chars().zip(text2.chars()) {
        if char1 == char2 {
            prefix_byte_len += char1.len_utf8();
        } else {
            break;
        }
    }
    &text1[..prefix_byte_len]
}

fn main() {
    let sentences = "This is my melody. And it's just, a ravers fantasy. All I know. If you in love with me tonight, we rave through the night.".to_string();

    let text = TextContext { text: &sentences };

    println!("The first sentences is {}.", text.get_first_sentence());

    let text1 = "This is the first string";
    let text2 = "This is the second string";
    println!(
        "The shared prefix of text1 and text2: {}",
        get_shared_prefix(text1, text2)
    );
}
