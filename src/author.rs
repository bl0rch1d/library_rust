use crate::entities::Author;

const NAME_LENGTH: usize = 20;
const BIO_LENGTH: usize = 140;

impl Author {
    pub fn new(name: String, bio: String) -> Author {
        if !Author::validate(&name, &bio) {
            panic!("Validation error");
        }

        Author {
            name: name,
            bio: bio
        }
    }

    pub fn to_s(self) -> String {
        format!("Name: {}. \nBio: {}", self.name, self.bio)
    }

    fn validate(name: &String, bio: &String) -> bool {
        let name_validation: bool = !name.is_empty() && name.len() <= NAME_LENGTH;
        let bio_validation: bool = !bio.is_empty() && bio.len() <= BIO_LENGTH;

        bio_validation && name_validation
    }
}
