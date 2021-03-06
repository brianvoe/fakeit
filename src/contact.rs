use crate::data::contact;
use crate::data::internet;
use crate::misc;
use crate::name;
use ::std::string::String;

pub struct Info {
    phone: String,
    email: String,
}

pub fn info() -> Info {
    Info {
        phone: phone_formatted(),
        email: email(),
    }
}

pub fn phone() -> String {
    misc::replace_with_numbers("##########".to_string())
}

pub fn phone_formatted() -> String {
    misc::replace_with_numbers(misc::random_data(contact::PHONE).to_string())
}

pub fn email() -> String {
    format!(
        "{}{}@{}.{}",
        name::first(),
        name::last(),
        name::last(),
        misc::random_data(internet::DOMAIN_SUFFIX).to_string()
    )
    .to_lowercase()
}

#[cfg(test)]
mod tests {
    use crate::contact;
    use crate::test_helper;

    #[test]
    fn phone() {
        let data1 = contact::phone();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn phone_formatted() {
        let data1 = contact::phone_formatted();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn email() {
        let data1 = contact::email();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}
