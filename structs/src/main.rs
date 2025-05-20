#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
        }
    }

    fn is_long_read(&self) -> bool {
        return self.pages > 500;
    }
}

fn describe_book(book: &Book) {
    println!(
        "{} by {} is {} pages long.",
        book.title, book.author, book.pages
    );
}

fn main() {
    println!("--- Part One ---");
    let shadow_slave = Book {
        title: String::from("ShadowSlave"),
        author: String::from("GuiltyThree"),
        pages: 20032,
    };
    println!("Title: {}", shadow_slave.title);
    println!("Author: {}", shadow_slave.author);
    println!("Pages: {}", shadow_slave.pages);

    println!("Book: \n {:#?}", shadow_slave);
    describe_book(&shadow_slave);

    println!("\n\n---Part Two ---");

    let short_book = Book::new(
        "Lord Of The Mysteries".to_string(),
        "Pupplefish".to_string(),
        300,
    );
    describe_book(&short_book);
    println!(
        "Is {} long read: {}.",
        short_book.title,
        short_book.is_long_read()
    );

    let long_book = Book::new("Sapiens".to_string(), "Yuval Noah Harrari".to_string(), 800);
    describe_book(&long_book);
    println!(
        "Is {} long read: {}.",
        long_book.title,
        long_book.is_long_read()
    );
}
