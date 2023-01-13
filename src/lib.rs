/// Type-erased errors
pub type BoxError = std::boxed::Box<
    dyn std::error::Error // Must implement `Error` to satisfy `?`
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;

struct Rollable {
    amount: i32,
    sides: i32,
}

pub mod parsers {

    pub fn parse_roll(i: &str) -> nom::IResult<&str, &str> {
        nom::bytes::complete::tag("roll")(i)
    }

    // pub fn parse_dice(i: &str) -> nom::IResult<&str, &str> {}

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_roll() {
            assert_eq!(parse_roll("roll 2d8"), Ok((" 2d8", "roll")));
            assert_eq!(
                parse_roll("toll 2d8"),
                Err(nom::Err::Error(nom::error::Error::new(
                    "toll 2d8",
                    nom::error::ErrorKind::Tag
                )))
            );
        }
    }
}
