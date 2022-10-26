//remove pub
pub struct Query(pub Vec<Condition>);

pub enum Condition {
    OracleText(String),
    ManaCost(Relation, usize)
}

pub enum Relation {
    LessThan,
    Equal,
    NotEqual,
    GreaterThan,
}
