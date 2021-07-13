use crate::entities::{Book, Author};

const TITLE_LENGTH: usize = 50;

impl Book {
    pub fn new(title: String, author: &Author) -> Book {
        if !Book::validate(&title) {
            panic!("Validation error");
        }

        Book {
            title: title,
            author: author.clone()
        }
    }

    pub fn to_s(self) {
        println!("Book title: {}. \nAuthor info:\n{}", self.title, self.author.to_s())
    }

    fn validate(title: &String) -> bool {
        !title.is_empty() && title.len() <= TITLE_LENGTH
    }
}
