use crate::card::Card;
#[macro_use] mod macros;
use crate::card_list::CardData;

pub fn parse(json_file: Vec<u8>) -> Result<CardData, anyhow::Error> {
    let cards_json = json::parse(&std::str::from_utf8(&json_file)?)?;
    let mut cards = Vec::new();
    let mut errs = 0u32;
    for card_json in cards_json.members() {
        let card = Card::new(card_json);
        match card {
            Ok(c) => cards.push(c),
            Err(e) => {
                eprintln!("{}",  e);
           //     match e{
           //         crate::card::error::Error::MissingField(_) => {println!("{}",  card_json.pretty(4));},
           //         _ => {}
           //     }
                errs+=1}
        }
    }
    eprintln!("{} parsed sucessfully, {} failed to parse",  cards.len(), errs);
    Ok(CardData::new(cards))
}
