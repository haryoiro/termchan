use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Menu {
    Table,
    Id,
    Url,
    Name,
}

#[derive(Iden)]
pub enum Category {
    Table,
    Id,
    MenuId,
    Name,
}

#[derive(Iden)]
pub enum Board {
    Table,
    Id,
    Name,
    Url,
    MenuId,
    CategoryId,
}

#[derive(Iden)]
pub enum BoardBookmark {
    Table,
    Id,
    BoardId,
    CategoryId,
    MenuId,
    Rating,
}

#[derive(Iden)]
pub enum Thread {
    Table,
    Id,
    Url,
    Name,
    Count,
    Ikioi,
    UpdatedAt,
    BoardId,
}

#[derive(Iden)]
pub enum ThreadPost {
    Table,
    Id,
    /// スレッドID データベースのThreadテーブルに対応
    ThreadId,
    /// ランダムなユーザーID
    /// データベースのリレーションとは関係ない
    PostId,
    /// 投稿番号
    Index,
    /// ユーザネーム
    Name,
    /// sageとかageとか
    Email,
    /// unix time
    Date,
    /// 投稿内容
    Message,
}
#[derive(Iden)]
pub enum Image {
    Table,
    Id,
    Url,
    SavePath,
    Blob,
}
