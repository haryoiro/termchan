use super::url::URL;

#[derive(Debug, Clone)]
pub struct BoardParams {
    pub url:       String,
    pub scheme:    String,
    pub host:      String,
    pub board_key: String,
}

impl From<&str> for BoardParams {
    fn from(url: &str) -> Self {
        let origin_url = url.clone();
        let mut spurl = url.split("/");
        let mut scheme = spurl.next().unwrap().to_string();
        scheme.pop();
        spurl.next(); // ""
        let host = spurl.next().unwrap().to_string();
        let board_key = spurl.next().unwrap().to_string();

        Self {
            url: origin_url.to_string(),
            scheme,
            host,
            board_key,
        }
    }
}

impl URL for BoardParams {
    fn new(url: &str) -> Self {
        Self::from(url)
    }
    fn origin(&self) -> String {
        format!("{}://{}", self.scheme, self.host)
    }
    fn host(&self) -> String {
        format!("{}", self.host)
    }
    fn referer(&self) -> String {
        format!("{}://{}/{}/", self.scheme, self.host, self.board_key)
    }
}

impl BoardParams {
    pub fn build_post(&self) -> String {
        format!("{}://{}/test/bbs.cgi", self.scheme, self.host)
    }
    pub fn build_board_url(&self) -> String {
        format!("{}://{}/{}/", self.scheme, self.host, self.board_key)
    }
}
