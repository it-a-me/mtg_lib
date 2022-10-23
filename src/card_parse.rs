pub mod card;
use card::Card;

pub fn parse(json_file: Vec<u8>) -> Result<Vec<Card>, anyhow::Error> {
    let cards_json = json::parse(&std::str::from_utf8(&json_file)?)?;
    let mut cards = Vec::new();
    let mut errs = Vec::new();
    for card in cards_json.members() {
        let card = Card::new(card);
        match card {
            Ok(c) => cards.push(c),
            Err(e) => errs.push(e)
        }
    }
    Ok(cards)
}
