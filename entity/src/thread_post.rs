//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "thread_post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id:        i32,
    pub index:     i32,
    pub name:      String,
    pub email:     String,
    pub post_id:   String,
    pub message:   String,
    pub date:      Option<String>,
    pub thread_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::thread::Entity",
        from = "Column::ThreadId",
        to = "super::thread::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Thread,
}

impl Related<super::thread::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Thread.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
}
