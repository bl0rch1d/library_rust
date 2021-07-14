mod entities;
mod author;
mod book;
mod reader;
mod order;
mod library;
mod storage;

use entities::*;
use library::Entity;
use storage::Storage;


fn main() {
    // let author = Author::new("Steven King".to_string(), "Some bio for author".to_string());
    // let book = Book::new("Steven King".to_string(), &author);
    // let reader = Reader::new(
    //     "Fool".to_string(),
    //     "yofool@mail.com".to_string(),
    //     "Foolish city".to_string(),
    //     "Foolish street".to_string(),
    //     "Fool's house".to_string()
    // );
    // let order = Order::new(&book, &reader, "12.12.2600".to_string());

    // println!("{}", author.to_s());
    // println!("{}", book.to_s());
    // println!("{}", reader.to_s());
    // println!("{}", order.to_s());

    let mut library = Library::new(vec![], vec![], vec![], vec![]);
    // library.add_entity(Entity::Author(&author));
    // library.add_entity(Entity::Author(&author));
    // library.add_entity(Entity::Book(&book));
    // library.add_entity(Entity::Order(&order));
    // library.add_entity(Entity::Reader(&reader));
    // library.add_entity(Entity::Reader(&reader));
    // library.add_entity(Entity::Reader(&reader));
    library.load_data();

    println!("{:?}", library.authors);
    println!("\n============\n");
    println!("{:?}", library.books);
    println!("\n============\n");
    println!("{:?}", library.orders);
    println!("\n============\n");
    println!("{:?}", library.readers);

    // library.save_data();
    // library.load_data();
}
