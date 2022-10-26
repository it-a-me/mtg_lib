use crate::card::Card;

pub struct CardList<'a>(Vec<&'a Card>);
impl<'a> std::ops::Index<usize> for CardList<'a> {
    type Output = Card;
    fn index<'b>(&'b self, i: usize) -> &'b Card {
        self.0[i]
    }
}
impl<'a> CardList<'a> {
    pub fn filter(&self, query: crate::query::Query) -> Self {
        use crate::query::Condition;
        use regex::Regex;
        let mut current_valid = self.0.clone();
        for condition in query.0 {
            match condition {
                Condition::OracleText(s) => {
                    use std::str::FromStr;
                    eprintln!("remove unwrap(card list) and pub from card.face card_face.oracle_text");
                    let re = Regex::from_str(&s).unwrap();
                    current_valid = current_valid.into_iter().filter(|c| re.is_match(&c.faces[0].oracle_text)).collect::<Vec<&Card>>();
                }
                _ => {panic!()}
            }
        }
        CardList(current_valid)

        //&self.0.iter().map(|c|
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardData(Vec<Card>);

impl CardData {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(&self)
    }
    pub fn deserialize(bytes: &Vec<u8>) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }

    pub fn mk_list(&self) -> CardList {
        CardList(self.0.iter().map(|c| c).collect())
    }
}
