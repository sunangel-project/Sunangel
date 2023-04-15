use anyhow::{anyhow, bail, Error, Result};

fn quarter_circle_card(input: String) -> Result<f64, Error> {
    match input.as_str() {
        "N" => Ok(0.),
        "NNE" => Ok(22.5),
        "NE" => Ok(45.),
        "ENE" => Ok(67.5),
        "E" => Ok(90.),
        _ => Err(anyhow!("this should not happen, '{input}' is invalid")),
    }
}

fn half_circle_card(input: String) -> Result<f64, Error> {
    if input.contains('S') {
        Ok(180. - quarter_circle_card(input.replace('S', "N"))?)
    } else {
        quarter_circle_card(input)
    }
}

fn full_circle_card(input: String) -> Result<f64, Error> {
    if input.contains('W') {
        Ok(360. - half_circle_card(input.replace('W', "E"))?)
    } else {
        half_circle_card(input)
    }
}

fn parse_direction_cardinals(input: &str) -> Result<f64, Error> {
    let in_upper = input.to_uppercase();

    if in_upper.is_empty()
        || in_upper.len() > 3
        || in_upper
            .chars()
            .any(|cardinal| !vec!['N', 'E', 'S', 'W'].contains(&cardinal))
    {
        bail!("invalid cardinals '{input}'");
    }

    full_circle_card(in_upper)
}

fn parse_direction(input: &str) -> Result<f64, Error> {
    str::parse::<f64>(input).or(parse_direction_cardinals(input))
}

pub fn direction_from_string(input: &str) -> Result<f64, Error> {
    if input.is_empty() {
        bail!("empty input for cardinal direction parsing")
    } else {
        parse_direction(input)
    }
}

#[cfg(test)]
mod tests {
    use super::direction_from_string;

    #[test]
    fn empty() {
        direction_from_string("").expect_err("error expected");
    }

    #[test]
    fn int() {
        assert_eq!(10., direction_from_string("10").unwrap())
    }

    #[test]
    fn float() {
        assert_eq!(0.5, direction_from_string("0.5").unwrap())
    }

    #[test]
    fn card_4() {
        assert_eq!(0., direction_from_string("N").unwrap());
        assert_eq!(90., direction_from_string("E").unwrap());
        assert_eq!(180., direction_from_string("S").unwrap());
        assert_eq!(270., direction_from_string("W").unwrap());
    }

    #[test]
    fn card_8() {
        assert_eq!(45., direction_from_string("NE").unwrap());
        assert_eq!(315., direction_from_string("NW").unwrap());
        assert_eq!(135., direction_from_string("SE").unwrap());
        assert_eq!(225., direction_from_string("SW").unwrap());
    }

    #[test]
    fn card_16() {
        assert_eq!(67.5, direction_from_string("ENE").unwrap());
        assert_eq!(157.5, direction_from_string("SSE").unwrap());
        assert_eq!(337.5, direction_from_string("NNW").unwrap());
    }
}
