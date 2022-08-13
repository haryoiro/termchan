//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "thread")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id:           i32,
    pub index:        i32,
    pub name:         String,
    pub url:          String,
    pub count:        i32,
    pub ikioi:        Option<f64>,
    pub stopdone:     bool,
    pub is_read:      bool,
    pub before_read:  i32,
    pub created_time: Option<i64>,
    pub board_id:     i32,
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
    fn before_save(self, insert: bool) -> Result<Self, DbErr> {
        let mut s = self;
        let is_read = s.clone().is_read.unwrap();
        if insert && is_read {
            s.before_read = s.count.clone();
        }
        Ok(s)
    }
}
