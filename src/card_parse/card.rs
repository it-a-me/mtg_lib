pub mod error;

macro_rules! get_field {
    ($json:expr, $field:expr, $json_into:path) => {{
        let field = &$json[$field];
        match $json_into(field) {
            Some(f) => Ok(f.to_owned()),
            None => Err(error::Error::MissingField($field)),
        }
    }};
}

macro_rules! get_option_field {
    ($json:expr, $field:expr, $json_into:path) => {{
        let field = &$json[$field];
        match $json_into(field) {
            Some(f) => Some(f.to_owned()),
            None => None,
        }
    }};
}

#[derive(Debug)]
pub struct Card {
    name: String,
    oracle_id: String,
    id: String,
    release_date: String,
    uri: String,
    scryfall_uri: String,
    image_uris: Option<ImageUris>,
    formats: Formats,
    type_line: String,
    faces: Vec<CardFace>
}

#[derive(Debug)]
pub struct CardFace {
    name:String,
    oracle_text:String,
    mana_cost:String,
    type_line:String,
    types:Types,
    image_uris: Option<ImageUris>,
}
impl CardFace {
    pub fn new(face_json: &json::JsonValue) -> Result<Self, error::Error> {
        let type_line = get_field!(&face_json, "type_line", json::JsonValue::as_str)?;
        Ok(Self {
            name: get_field!(&face_json, "name", json::JsonValue::as_str)?,
            types: Types::new(&type_line),
            mana_cost: get_field!(&face_json, "mana_cost", json::JsonValue::as_str)?,
            type_line,
            oracle_text: get_field!(&face_json, "oracle_text", json::JsonValue::as_str)?,
            image_uris: ImageUris::new(&face_json["image_uris"])?,
        })
    }
}

impl Card {
    pub fn new(card_json: &json::JsonValue) -> Result<Self, error::Error> {
        if !Self::is_card(&card_json) {
            return Err(error::Error::NotACard);
        }

        let mut faces = Vec::new();
        if card_json.has_key("card_faces") {
            for face in card_json["card_faces"].members() {
                faces.push(CardFace::new(face)?);
            }
        } else {
            faces.push(CardFace::new(&card_json)?);
        }
        Ok(Self {
            name: get_field!(&card_json, "name", json::JsonValue::as_str)?,
            oracle_id: get_field!(&card_json, "oracle_id", json::JsonValue::as_str)?,
            id: get_field!(&card_json, "id", json::JsonValue::as_str)?,
            release_date: get_field!(&card_json, "released_at", json::JsonValue::as_str)?,
            uri: get_field!(&card_json, "uri", json::JsonValue::as_str)?,
            scryfall_uri: get_field!(&card_json, "scryfall_uri", json::JsonValue::as_str)?,
            formats: Formats::new(&card_json["legalities"])?,
            type_line: get_field!(&card_json, "type_line", json::JsonValue::as_str)?,
            image_uris: ImageUris::new(&card_json["image_uris"])?,
            faces,
        })
    }
    fn is_card(card_json: &json::JsonValue) -> bool {
        let object_type = &card_json["object"];
        if let Some(t) = object_type.as_str() {
            t == "card"
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct ImageUris {
    small: String,
    normal: String,
    large: String,
    png: String,
    art_crop: String,
    border_crop: String,
}
impl ImageUris {
    pub fn new(uri_json: &json::JsonValue) -> Result<Option<Self>, error::Error> {
        if uri_json.is_empty() {
            return Ok(None);
        }
        Ok(Some(Self {
            small: get_field!(&uri_json, "small", json::JsonValue::as_str)?,
            normal: get_field!(&uri_json, "normal", json::JsonValue::as_str)?,
            large: get_field!(&uri_json, "large", json::JsonValue::as_str)?,
            png: get_field!(&uri_json, "png", json::JsonValue::as_str)?,
            art_crop: get_field!(&uri_json, "art_crop", json::JsonValue::as_str)?,
            border_crop: get_field!(&uri_json, "border_crop", json::JsonValue::as_str)?,
        }))
    }
}

#[derive(Debug)]
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
#[derive(Debug)]
struct Formats {
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

#[derive(Debug)]
struct Types {
    super_types: Vec<String>,
    subtypes: Vec<String>,
}

impl Types {
    pub fn new(type_line: &str) -> Self {
        let (super_t, sub_t) = match type_line.split_once('—') {
            Some(t) => t,
            None => (type_line, ""),
        };
        Self {
            super_types: super_t
                .split_whitespace()
                .map(|t| t.trim().to_owned())
                .collect(),
            subtypes: sub_t
                .split_whitespace()
                .map(|t| t.trim().to_owned())
                .collect(),
        }
    }
}
