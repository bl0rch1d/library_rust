mod entities;
mod author;
mod book;
mod reader;
mod order;

use entities::*;


fn main() {
    let author = Author::new("Steven King".to_string(), "Some bio for author".to_string());
    let book = Book::new("Steven King".to_string(), &author);
    let reader = Reader::new(
        "Fool".to_string(),
        "yofool@mail.com".to_string(),
        "Foolish city".to_string(),
        "Foolish street".to_string(),
        "Fool's house".to_string()
    );
    let order = Order::new(&book, &reader, "12.12.2600".to_string());

    // println!("{}", author.to_s());
    // println!("{}", book.to_s());
    // println!("{}", reader.to_s());
    println!("{}", order.to_s());
}
