use super::error;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Cost {
    white: u32,
    blue: u32,
    black: u32,
    red: u32,
    green: u32,
    colorless: u32,
    generic: u32,
    x: u32,
}
impl Cost {
    pub fn new(card_json: &json::JsonValue) -> Result<Self, error::Error> {
        lazy_static! {
            static ref MANARE: Regex = Regex::new(r"\{(.*?)\}").unwrap();
        }
        let cost_string = get_field!(&card_json, "mana_cost", json::JsonValue::as_str)?;
        let mut cost = Self::blank();
        for mana in MANARE.captures_iter(&cost_string) {
            match mana[1].to_ascii_lowercase().as_str() {
                "w" => cost.white += 1,
                "u" => cost.blue += 1,
                "b" => cost.black += 1,
                "r" => cost.red += 1,
                "g" => cost.green += 1,
                "c" => cost.colorless += 1,
                "x" => cost.x += 1,
                _ => match mana[1].parse::<u32>() {
                    Ok(g) => cost.generic += g,
                    Err(_) => {
                        return Err(error::Error::InvalidColor(mana[1].to_owned()));
                    }
                },
            }
        }
        Ok(cost)
    }

    fn blank() -> Self {
        Self {
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            colorless: 0,
            generic: 0,
            x: 0,
        }
    }
}
