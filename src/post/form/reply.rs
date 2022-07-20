use std::collections::HashMap;

use crate::{
    url::reply::ThreadParams,
    util::{encoding::utf8_to_sjis_string, time::unix_now_time},
};

pub struct ReplyFormData {
    name:           String,
    mail:           String,
    message:        String,
    bbs:            String,
    key:            String,
    time:           String,
    submit:         String,
    oekaki_thread1: String,
}

impl ReplyFormData {
    pub fn new(
        message: &str,
        email: Option<&str>,
        name: Option<&str>,
        thread_params: &ThreadParams,
    ) -> Self {
        let time = unix_now_time().to_string();
        let name = name.unwrap_or("").to_string();
        let mail = email.unwrap_or("").to_string();
        ReplyFormData {
            name,
            mail,
            message: message.to_string(),
            bbs: thread_params.board_key.to_string(),
            key: thread_params.thread_id.to_string(),
            time,
            submit: "書き込む".to_string(),
            oekaki_thread1: "".to_string(),
        }
    }

    pub fn build(&self) -> String {
        let mut form = HashMap::new();
        form.insert("FORM", &self.name);
        form.insert("mail", &self.mail);
        form.insert("MESSAGE", &self.message);
        form.insert("bbs", &self.bbs);
        form.insert("key", &self.key);
        form.insert("time", &self.time);
        form.insert("submit", &self.submit);
        form.insert("oekaki_thread1", &self.oekaki_thread1);
        form.iter_mut()
            .map(|(k, v)| (*k, utf8_to_sjis_string(v)))
            .collect::<HashMap<_, _>>()
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }
}
