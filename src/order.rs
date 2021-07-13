use crate::entities::{Order, Book, Reader};

impl Order {
    pub fn new(book: &Book, reader: &Reader, date: String) -> Order {
        Order {
            book: book.clone(),
            reader: reader.clone(),
            date: date
        }
    }

    pub fn to_s(self) -> String {
        format!(
            "Date: {}\n===\nBook info:\n{}\n===\nReader info:\n{}\n",
            self.date,
            self.book.to_s(),
            self.reader.to_s()
        )
    }
}
