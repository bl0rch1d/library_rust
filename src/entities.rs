#[derive(Clone, Debug)]
pub struct Author {
    pub name: String,
    pub bio: String
}

#[derive(Clone, Debug)]
pub struct Book {
    pub author: Author,
    pub title: String
}

#[derive(Clone, Debug)]
pub struct Reader {
    pub name: String,
    pub email: String,
    pub city: String,
    pub street: String,
    pub house: String
}

#[derive(Clone, Debug)]
pub struct Order {
    pub book: Book,
    pub reader: Reader,
    pub date: String
}

pub struct Library {
    pub authors: Vec<Author>,
    pub books: Vec<Book>,
    pub readers: Vec<Reader>,
    pub orders: Vec<Order>,
}
