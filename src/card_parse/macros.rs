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
