use super::error;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Legalities {
    NotLegal,
    Restricted,
    Banned,
    Legal,
}
impl Legalities {
    fn new(l: &str) -> Result<Self, error::Error> {
        match l {
            "not_legal" => Ok(Legalities::NotLegal),
            "restricted" => Ok(Legalities::Restricted),
            "banned" => Ok(Legalities::Banned),
            "legal" => Ok(Legalities::Legal),
            _ => Err(error::Error::InvalidLegality(l.to_owned())),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(super) struct Formats {
    standard: Legalities,
    future: Legalities,
    historic: Legalities,
    gladiator: Legalities,
    pioneer: Legalities,
    explorer: Legalities,
    modern: Legalities,
    legacy: Legalities,
    pauper: Legalities,
    vintage: Legalities,
    penny: Legalities,
    commander: Legalities,
    brawl: Legalities,
    historicbrawl: Legalities,
    alchemy: Legalities,
    paupercommander: Legalities,
    duel: Legalities,
    oldschool: Legalities,
    premodern: Legalities,
}
impl Formats {
    pub fn new(legality_json: &json::JsonValue) -> Result<Self, error::Error> {
        Ok(Self {
            standard: Legalities::new(&get_field!(
                &legality_json,
                "standard",
                json::JsonValue::as_str
            )?)?,
            future: Legalities::new(&get_field!(
                &legality_json,
                "future",
                json::JsonValue::as_str
            )?)?,
            historic: Legalities::new(&get_field!(
                &legality_json,
                "historic",
                json::JsonValue::as_str
            )?)?,
            gladiator: Legalities::new(&get_field!(
                &legality_json,
                "gladiator",
                json::JsonValue::as_str
            )?)?,
            pioneer: Legalities::new(&get_field!(
                &legality_json,
                "pioneer",
                json::JsonValue::as_str
            )?)?,
            explorer: Legalities::new(&get_field!(
                &legality_json,
                "explorer",
                json::JsonValue::as_str
            )?)?,
            modern: Legalities::new(&get_field!(
                &legality_json,
                "modern",
                json::JsonValue::as_str
            )?)?,
            legacy: Legalities::new(&get_field!(
                &legality_json,
                "legacy",
                json::JsonValue::as_str
            )?)?,
            pauper: Legalities::new(&get_field!(
                &legality_json,
                "pauper",
                json::JsonValue::as_str
            )?)?,
            vintage: Legalities::new(&get_field!(
                &legality_json,
                "vintage",
                json::JsonValue::as_str
            )?)?,
            penny: Legalities::new(&get_field!(
                &legality_json,
                "penny",
                json::JsonValue::as_str
            )?)?,
            commander: Legalities::new(&get_field!(
                &legality_json,
                "commander",
                json::JsonValue::as_str
            )?)?,
            brawl: Legalities::new(&get_field!(
                &legality_json,
                "brawl",
                json::JsonValue::as_str
            )?)?,
            historicbrawl: Legalities::new(&get_field!(
                &legality_json,
                "historicbrawl",
                json::JsonValue::as_str
            )?)?,
            alchemy: Legalities::new(&get_field!(
                &legality_json,
                "alchemy",
                json::JsonValue::as_str
            )?)?,
            paupercommander: Legalities::new(&get_field!(
                &legality_json,
                "paupercommander",
                json::JsonValue::as_str
            )?)?,
            duel: Legalities::new(&get_field!(
                &legality_json,
                "duel",
                json::JsonValue::as_str
            )?)?,
            oldschool: Legalities::new(&get_field!(
                &legality_json,
                "oldschool",
                json::JsonValue::as_str
            )?)?,
            premodern: Legalities::new(&get_field!(
                &legality_json,
                "premodern",
                json::JsonValue::as_str
            )?)?,
        })
    }
}

