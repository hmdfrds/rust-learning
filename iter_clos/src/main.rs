struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let books = vec![
        Book {
            title: "Shadow Slave".to_string(),
            pages: 800,
        },
        Book {
            title: "LOTM".to_string(),
            pages: 500,
        },
        Book {
            title: "Omniscient Reader".to_string(),
            pages: 1000,
        },
        Book {
            title: "Solo Levelling".to_string(),
            pages: 200,
        },
    ];

    println!("\n\n--- Part 1 ---");
    books
        .iter()
        .filter(|b| b.pages > 500)
        .for_each(|b| println!("{} Has more than 500 pages.", b.title));

    println!("\n\n--- Part 2 ---");
    let page_sum: u32 = books.iter().map(|b| b.pages).sum();
    println!("All books have {} pages", page_sum);

    println!("\n\n--- Part 3 ---");
    let new_books: Vec<String> = books
        .iter()
        .map(|b| -> String {
            let mut title = b.title.clone();
            if b.pages > 300 {
                title.push_str(" (Long Read)");
            } else {
                title.push_str(" (Short Read)");
            }
            title
        })
        .collect();

    new_books.iter().for_each(|t| println!("{}", t));
}
