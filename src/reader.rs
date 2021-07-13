use crate::entities::Reader;

const NAME_LENGTH: usize = 20;
const EMAIL_LENGTH: usize = 50;
const COMMON_STRING_LENGTH: usize = 60;

impl Reader {
    pub fn new(name: String, email: String, city: String, street: String, house: String) -> Reader {
        if !Reader::validate(&name, &email, &city, &street, &house) {
            panic!("Validation error");
        }

        Reader {
            name: name,
            email: email,
            city: city,
            street: street,
            house: house
        }
    }

    pub fn to_s(self) -> String {
        format!(
            "Name: {}\nEmail: {}\nCity: {}\nStreet: {}\nHouse: {}\n",
            self.name,
            self.email,
            self.city,
            self.street,
            self.house
        )
    }

    fn validate(name: &String, email: &String, city: &String, street: &String, house: &String) -> bool {
        let name_validation: bool = !name.is_empty() && name.len() <= NAME_LENGTH;
        let email_validation: bool = !email.is_empty() && email.len() <= EMAIL_LENGTH;
        let city_validation: bool = !city.is_empty() && city.len() <= COMMON_STRING_LENGTH;
        let street_validation: bool = !street.is_empty() && street.len() <= COMMON_STRING_LENGTH;
        let house_validation: bool = !house.is_empty() && house.len() <= COMMON_STRING_LENGTH;

        name_validation &&
            email_validation &&
            city_validation &&
            street_validation &&
            house_validation
    }
}
