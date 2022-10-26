pub mod error;
mod formats;
//mod mana;
use formats::Formats;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
    pub faces: Vec<CardFace>,
    cmc:f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CardFace {
    name:String,
    pub oracle_text:String,
    mana_cost:String,
    type_line:String,
    types:Types,
    image_uris: Option<ImageUris>,
    power: Option<String>,
    toughness: Option<String>,
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
            power: get_option_field!(&face_json, "power", json::JsonValue::as_str),
            toughness: get_option_field!(&face_json, "toughness", json::JsonValue::as_str),
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
            cmc: get_field!(&card_json, "cmc", json::JsonValue::as_f32)?,
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
