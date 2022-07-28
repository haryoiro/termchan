//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "thread")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id:         i32,
    pub name:       String,
    pub url:        String,
    pub count:      i32,
    pub ikioi:      Option<f64>,
    pub updated_at: Option<String>,
    pub board_id:   i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::board::Entity",
        from = "Column::BoardId",
        to = "super::board::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Board,
    #[sea_orm(has_many = "super::thread_post::Entity")]
    ThreadPost,
}

impl Related<super::board::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Board.def()
    }
}

impl Related<super::thread_post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThreadPost.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
}
