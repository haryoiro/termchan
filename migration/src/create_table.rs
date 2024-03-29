use sea_orm_migration::prelude::*;

use crate::table::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Menu Schema
        manager
            .create_table(
                Table::create()
                    .table(Menu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Menu::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Menu::Url).string().not_null().unique_key())
                    .col(ColumnDef::new(Menu::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        // Category Schema
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(
                        ColumnDef::new(Category::MCategoryName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Category::MenuId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_category_menu_id")
                            .from(Category::Table, Category::MenuId)
                            .to(Menu::Table, Menu::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Board Schema
        manager
            .create_table(
                Table::create()
                    .table(Board::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Board::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Board::Url).string().not_null())
                    .col(ColumnDef::new(Board::Name).string().not_null())
                    .col(
                        ColumnDef::new(Board::MCBoardName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Board::MenuId).integer().not_null())
                    .col(ColumnDef::new(Board::CategoryId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_board_menu_id")
                            .from(Board::Table, Board::MenuId)
                            .to(Menu::Table, Menu::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_board_category_id")
                            .from(Board::Table, Board::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Thread Schema
        manager
            .create_table(
                Table::create()
                    .table(Thread::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Thread::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Thread::Index).integer().not_null())
                    .col(ColumnDef::new(Thread::Name).string().not_null())
                    .col(ColumnDef::new(Thread::Url).string().not_null().unique_key())
                    .col(ColumnDef::new(Thread::Count).integer().not_null())
                    .col(ColumnDef::new(Thread::Ikioi).float())
                    .col(
                        ColumnDef::new(Thread::IsRead)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Thread::Stopdone)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Thread::BeforeRead)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Thread::CreatedTime).integer())
                    .col(ColumnDef::new(Thread::BoardId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_thread_board_id")
                            .from(Thread::Table, Thread::BoardId)
                            .to(Board::Table, Board::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ThreadPost::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ThreadPost::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ThreadPost::Index).integer().not_null())
                    .col(ColumnDef::new(ThreadPost::Name).string().not_null())
                    .col(ColumnDef::new(ThreadPost::Email).string().not_null())
                    .col(ColumnDef::new(ThreadPost::PostId).string().not_null())
                    .col(ColumnDef::new(ThreadPost::Message).json().not_null())
                    .col(ColumnDef::new(ThreadPost::Date).string())
                    .col(ColumnDef::new(ThreadPost::ThreadId).integer().not_null())
                    .col(
                        ColumnDef::new(ThreadPost::ThreadIdIndex)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_thread_post_thread_id")
                            .from(ThreadPost::Table, ThreadPost::ThreadId)
                            .to(Thread::Table, Thread::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Image Schema
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Image::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Image::SavePath).string().not_null())
                    .col(ColumnDef::new(Image::Url).string().not_null().unique_key())
                    .col(ColumnDef::new(Image::Size).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(BoardBookmark::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BoardBookmark::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BoardBookmark::Rating).integer().not_null())
                    .col(
                        ColumnDef::new(BoardBookmark::BoardId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_board_bookmark_board_id")
                            .from(BoardBookmark::Table, BoardBookmark::BoardId)
                            .to(Board::Table, Board::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ThreadPost::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Thread::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Board::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BoardBookmark::Table).to_owned())
            .await
    }
}
