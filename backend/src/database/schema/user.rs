use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub user_id: i32,
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::file::Entity")]
    Files,
}

impl ActiveModelBehavior for ActiveModel {}