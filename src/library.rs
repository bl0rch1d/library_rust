use crate::entities::*;

pub enum Entity<'a> {
    Book(&'a Book),
    Author(&'a Author),
    Reader(&'a Reader),
    Order(&'a Order),
}

impl Library {
    pub fn new(books: Vec<Book>, authors: Vec<Author>, readers: Vec<Reader>, orders: Vec<Order>) -> Library {
        Library {
            authors: authors,
            books: books,
            readers: readers,
            orders: orders
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        match entity {
            Entity::Book(entity) => self.books.push(entity.clone()),
            Entity::Author(entity) => self.authors.push(entity.clone()),
            Entity::Reader(entity) => self.readers.push(entity.clone()),
            Entity::Order(entity) => self.orders.push(entity.clone())
        }
    }
}
