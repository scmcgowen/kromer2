use sea_query::Iden;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Iden)]
pub enum Player {
    Table,
    Id,
    Name,
}
