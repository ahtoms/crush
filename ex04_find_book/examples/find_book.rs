use std::io::{self, Read};

fn find_book(books: &Vec<String>, title_to_find: &str) -> Option<String> {
    for book in books {
        if title_to_find == book {
            return Some(String::from(book.as_ref()));
        }
    }
    None
}


fn main() {
    
    let books = vec![
        "To Kill a Mocking Bird".to_string(),
        "The Great Gatsby".to_string(),
        "Animal Farm".to_string(),
        "Lord of the Flies".to_string(),
        "The Catcher in the Rye".to_string(),
        "1984".to_string(),
        "Frankenstein".to_string(),
        "Pride and Prejudice".to_string()
    ];
    
    let mut line = String::new();
    while let Ok(bytes) = io::stdin().read_line(&mut line) {
        match find_book(&books, line.trim()) {
            Some(b) => { println!("Found: {}", b); line.clear(); }
            None => { println!("Did not find book :(" ); }
        }
    }
    
    
}
