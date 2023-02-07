//! libqizzer
//!
//! Utilities to represent and de/serialize Quiz Data
pub mod data;
pub mod quiz;

pub use data::*;
pub use quiz::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::OpenOptions;

    #[test]
    fn serializes() {
        let raw_values = vec![
            (1u8, "Do you love her?", "yes"),
            (1u8, "Amoogns cronut", "no"),
            (1u8, "Weeeeeeeeeeee", "yes"),
            (1u8, "Fourth question?", "yes"),
            (1u8, "Is the next question of the same type?", "no"),
            (2u8, "Who is the bestest doggo in the world?", "Rosie"),
            (2u8, "Who is the smelliest doggo in the world?", "Tasha"),
            (3u8, "What is life?", "Damn bro idk"),
            (
                3u8,
                "What is this hit game called Amongus?",
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            ),
        ];

        let file = OpenOptions::new().write(true).create(true).open("dump.bin");
        assert!(file.is_ok());
        let mut file = file.unwrap();

        let quiz = quiz::Quiz::from(raw_values);
        let res = data::dump(&mut file, &quiz);

        assert!(res.is_ok());
    }

    #[test]
    fn deserializes() {
        let raw_values = vec![
            (1u8, "Do you love her?", "yes"),
            (1u8, "Amoogns cronut", "no"),
            (1u8, "Weeeeeeeeeeee", "yes"),
            (1u8, "Fourth question?", "yes"),
            (1u8, "Is the next question of the same type?", "no"),
            (2u8, "Who is the bestest doggo in the world?", "Rosie"),
            (2u8, "Who is the smelliest doggo in the world?", "Tasha"),
            (3u8, "What is life?", "Damn bro idk"),
            (
                3u8,
                "What is this hit game called Amongus?",
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            ),
        ];

        let file = OpenOptions::new().read(true).open("dump.bin");
        assert!(file.is_ok());
        let mut file = file.unwrap();

        let quiz = quiz::Quiz::from(raw_values);

        let res = data::load(&mut file);
        if let Err(e) = &res {
            dbg!(e);
        }

        assert!(res.is_ok());
        assert_eq!(quiz, res.unwrap())
    }
}
