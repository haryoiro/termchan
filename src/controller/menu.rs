use anyhow::Context;
use regex::Regex;

use crate::{
    encoder::{is_utf8, sjis_to_utf8},
    receiver::Reciever,
};

fn normalize_bbsmenu(html: &mut String) -> anyhow::Result<Vec<BbsCategories>> {
    let html = Regex::new(r#" TARGET=(.*?)>"#)
        .context("failed to create regex")?
        .replace_all(&html, ">".to_string());

    let is_utf8 = html.contains("！");
    let html = if is_utf8 {
        Regex::new("！")
            .context("failed to create regex")?
            .replace_all(&html, "!".to_string())
    } else {
        html
    };

    let mut splitted: Vec<&str> = html.split("\n").collect::<Vec<&str>>();
    splitted.reverse();

    let mut lines = Vec::new();
    for line in splitted {
        if line.starts_with("<A HREF=") {
            // リンク
            if line.ends_with("</A><br>") {
                // 通常行
                lines.push(&line[8..line.len() - 8]);
            } else if line.ends_with("</A>") {
                // 最終行(<br>無し)
                lines.push(&line[8..line.len() - 4]);
            };
        } else if line.starts_with("<BR><BR><B>") || line.starts_with("<br><br><B>") {
            // カテゴリ名
            if line.ends_with("</B><BR>") || line.ends_with("</B><br>") {
                // 通常
                lines.push(&line[11..line.len() - 8]);
            } else {
                // BBSPINK
                // スライスだと全角文字が入るとエラーになるので,getを使用する
                let sliced = line
                    .get(11..line.len() - 44)
                    .context("failed to get slice")?;
                lines.push(sliced);
            };
        } else if !line.starts_with("<br><br>") {
            // コメント入りリンク
            let ll = line.split("<br>").collect::<Vec<&str>>();
            for l in ll {
                if l.starts_with("<A HREF=") && l.ends_with("</A><br>") {
                    lines.push(&l[8..l.len() - 5]);
                };
            }
        }
    } // for l in splitted

    let mut categories: Vec<BbsCategories> = Vec::new();
    let mut links: Vec<BbsUrl> = Vec::new();
    for l in lines {
        let s = l.split(">").collect::<Vec<&str>>();
        match s.len() {
            1 => {
                let category = s[0].to_string();
                categories.push(BbsCategories {
                    category,
                    list: links,
                });
                links = Vec::new();
            }
            2 => {
                let (url, title) = (s[0].to_string(), s[1].to_string());
                let title = if !is_utf8 {
                    let (title, ..) = encoding_rs::SHIFT_JIS.decode(title.as_bytes());
                    title.to_string()
                } else {
                    title
                };
                links.push(BbsUrl { title, url });
            }
            _ => {
                return Err(anyhow::anyhow!("failed to parse categories"));
            }
        }
    } // for l in lines

    // for c in &categories {
    //     println!("{}", c.category);
    //     for l in &c.list {
    //         println!("\t{} : {}", l.url, l.title);
    //     }
    // }

    Ok(categories)
}

#[derive(Debug)]
pub struct BbsMenu {
    pub url: String,
}

impl BbsMenu {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn load(&self) -> anyhow::Result<Vec<BbsCategories>> {
        let url = self.url.clone();
        let mut html = Reciever::get(&url).await.context("page error")?.html();
        Ok(normalize_bbsmenu(&mut html).context("failed to parse bbsmenu")?)
    }
}

#[derive(Debug)]
pub struct BbsCategories {
    pub category: String,
    pub list: Vec<BbsUrl>,
}

#[derive(Debug)]
pub struct BbsUrl {
    pub url: String,
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_5ch_bbsmenu() {
        let url = "https://menu.5ch.net/bbsmenu.html";
        let bbsmenu = BbsMenu::new(url.to_string());
        let result = bbsmenu.load().await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    // #[tokio::test]
    // async fn test_get_2ch_bbsmenu() {
    //     let url = "https://menu.2ch.sc/bbsmenu.html";
    //     let bbsmenu = Bbsmenu::new(url.to_string());
    //     let result = bbsmenu.load().await;
    //     println!("{:?}", result);
    //     assert!(result.is_ok());
    // }
}
