use crate::entities::Library;

use std::fs::File;
use std::io::Result;

const FILENAME: &str = "library.yml";

pub trait Storage {
    fn save_data(&self) -> Result<()>;
    fn load_data(&mut self) -> Result<()>;
}

impl Storage for Library {
    fn save_data(&self) -> Result<()> {
        let file = File::create(FILENAME)?;

        serde_yaml::to_writer(file, self);

        Ok(())
    }

    fn load_data(&mut self) -> Result<()> {
        let file = File::open(FILENAME)?;

        let loaded_data: Library = serde_yaml::from_reader(file).unwrap();

        self.orders = loaded_data.orders;
        self.books = loaded_data.books;
        self.readers = loaded_data.readers;
        self.authors = loaded_data.authors;

        Ok(())
    }
}
