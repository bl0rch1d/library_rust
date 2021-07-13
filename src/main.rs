mod entities;
mod author;

use entities::*;


fn main() {
    let author = Author::new("Steven King".to_string(), "Some bio for author".to_string());

    author.to_s();
}
