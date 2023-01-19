/// Type-erased errors
// pub type BoxError = std::boxed::Box<
//     dyn std::error::Error // Must implement `Error` to satisfy `?`
//         + std::marker::Send // needed for threads
//         + std::marker::Sync, // needed for threads
// >;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit0, one_of},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Rollable {
    amount: u32,
    sides: u32,
}

fn parse_roll(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("roll")(input)
}

/// {INT}d{INT}
fn parse_dice(input: &str) -> IResult<&str, Rollable> {
    match tuple((digit0, tag("d"), digit0))(input) {
        Ok((remaining_input, (amount, _, sides))) => {
            let amount: u32 = amount.parse().unwrap();
            let sides: u32 = sides.parse().unwrap();
            Ok((remaining_input, Rollable { amount, sides }))
        }
        Err(e) => Err(e),
    }
}

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

    #[test]
    fn test_parse_dice() {
        assert_eq!(
            parse_dice("2d8"),
            Ok((
                "",
                Rollable {
                    amount: 2,
                    sides: 8
                }
            ))
        );

        assert_eq!(
            parse_dice("qd8"),
            Err(nom::Err::Error(nom::error::Error::new(
                "qd8",
                nom::error::ErrorKind::Tag
            )))
        );

        assert_eq!(
            parse_dice("5dv"),
            Err(nom::Err::Error(nom::error::Error::new(
                "5dv",
                nom::error::ErrorKind::Tag
            )))
        );
    }
}
