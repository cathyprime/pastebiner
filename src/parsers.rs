pub(crate) mod list {
    use crate::utils::styles::{date, key, missing, privacy};
    use crate::utils::file_size::Size;
    use crate::utils::private::Privacy;
    use crate::utils::string_to_datetime::string_to_datetime;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use std::fmt::Display;
    use url::Url;

    #[derive(Deserialize, Debug)]
    pub(crate) struct Paste {
        paste_key: String,
        #[serde(deserialize_with = "string_to_datetime")]
        paste_date: DateTime<Utc>,
        paste_title: String,
        paste_size: Size,
        #[serde(deserialize_with = "string_to_datetime")]
        paste_expire_date: DateTime<Utc>,
        paste_private: Privacy,
        paste_format_long: String,
        paste_url: Url,
        paste_hits: i32,
    }

    impl Display for Paste {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let title = match self.paste_title.as_str() {
                "" => &missing().paint("<No title>").to_string(),
                _ => &self.paste_title,
            };
            writeln!(
                f,
                "{}",
                [
                    format!("title:       {}", title),
                    format!("key:         {}", key().paint(&self.paste_key)),
                    format!("url:         {}", self.paste_url),
                    format!("size:        {}", self.paste_size),
                    format!(
                        "privacy:     {}",
                        privacy().paint(self.paste_private.to_string())
                    ),
                    format!("format:      {}", self.paste_format_long),
                    format!("hits:        {}", self.paste_hits),
                    format!("date:        {}", date().paint(self.paste_date.to_string())),
                    format!(
                        "expire date: {}",
                        date().paint(self.paste_expire_date.to_string())
                    ),
                ]
                .join("\n")
            )
        }
    }
}

pub(crate) mod info {
    use crate::utils::styles::{missing, privacy};
    use crate::utils::private::Privacy;
    use crate::utils::website::Website;
    use ansi_term::{Color, Style};
    use serde::Deserialize;
    use std::fmt::Display;

    #[derive(Deserialize, Debug)]
    pub(crate) struct Info {
        user_name: String,
        #[serde(rename(deserialize = "user_format_short"))]
        _user_format_short: String,
        #[serde(rename(deserialize = "user_expiration"))]
        _user_expiration: String,
        user_private: Privacy,
        user_website: Website,
        user_email: String,
        user_location: String,
        user_account_type: i8,
    }

    impl Display for Info {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let underline = Style::new().underline();
            let location = match self.user_location.as_str() {
                "" => missing().paint("<No location>").to_string(),
                _ => self.user_location.to_string(),
            };
            let account_type = match self.user_account_type {
                0 => Color::Green.paint("normal").to_string(),
                1 => Color::Green.paint("pro").to_string(),
                _ => missing().paint("<No account type>").to_string(),
            };
            writeln!(
                f,
                "{}",
                [
                    format!(
                        "username:     {}",
                        Style::new().bold().paint(&self.user_name)
                    ),
                    format!("account type: {}", account_type),
                    format!("email:        {}", underline.paint(&self.user_email)),
                    format!("website link: {}", self.user_website),
                    format!("location:     {}", location),
                    format!(
                        "privacy:      {}",
                        privacy().paint(self.user_private.to_string())
                    ),
                ]
                .join("\n")
            )
        }
    }
}
