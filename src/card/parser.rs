use super::error::Error;
macro_rules! get_field {
    ($name:ident, $field:expr, $json_type:path, $out_type:ty) => {
        pub(super) fn $name(card_json: &json::JsonValue) -> Result<$out_type, Error> {
            let field = &card_json[$field];
            match $json_type(field) {
                Some(f) => Ok(f.to_owned()),
                None => Err(Error::MissingField($field)),
            }
        }
    };
}

impl super::Card {
    pub(super) fn is_card(card_json: &json::JsonValue) -> bool {
        let object_type = &card_json["object"];
        if let Some(t) = object_type.as_str() {
            t == "card"
        } else {
            false
        }
    }
    //get_field!(get_name, "name", json::JsonValue::as_str, String);
    get_field!(get_oracle_id, "oracle_id", json::JsonValue::as_str, String);
    get_field!(get_id, "id", json::JsonValue::as_str, String);
}


