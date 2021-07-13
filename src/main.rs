mod entities;
mod author;
mod book;

use entities::*;


fn main() {
    let author = Author::new("Steven King".to_string(), "Some bio for author".to_string());
    let book = Book::new("Steven King".to_string(), &author);

    author.to_s();
    book.to_s();
}
