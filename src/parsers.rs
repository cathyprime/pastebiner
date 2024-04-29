pub(super) mod list {
    use crate::utils::file_size::Size;
    use crate::utils::private::Privacy;
    use crate::utils::string_to_datetime::string_to_datetime;
    use ansi_term::Color;
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
                "" => &Color::Red.paint("<no title>").to_string(),
                _ => &self.paste_title,
            };
            writeln!(
                f,
                "{}",
                [
                    format!("title:       {}", title),
                    format!("key:         {}", Color::Red.bold().paint(&self.paste_key)),
                    format!("url:         {}", self.paste_url),
                    format!("size:        {}", self.paste_size),
                    format!("privacy:     {}", self.paste_private),
                    format!("format:      {}", self.paste_format_long),
                    format!("hits:        {}", self.paste_hits),
                    format!(
                        "date:        {}",
                        Color::Green.paint(self.paste_date.to_string())
                    ),
                    format!(
                        "expire date: {}",
                        Color::Green.paint(self.paste_expire_date.to_string())
                    ),
                ]
                .join("\n")
            )
        }
    }
}

/* <user>
 *         <user_name>Cathyprime47</user_name>
 *         <user_format_short>text</user_format_short>
 *         <user_expiration>N</user_expiration>
 *         <user_avatar_url>@themes/img/guest.png</user_avatar_url>
 *         <user_private>0</user_private>
 *         <user_website></user_website>
 *         <user_email>yoolayna47@gmail.com</user_email>
 *         <user_location></user_location>
 *         <user_account_type>0</user_account_type>
 * </user>
 */
pub(crate) mod info {
    use serde::Deserialize;
    use std::fmt::Display;
    use crate::utils::website::Website;

    #[derive(Deserialize, Debug)]
    pub(crate) struct Info {
        user_name: String,
        user_format_short: String,
        user_expiration: String,
        user_private: String,
        user_website: Website,
        user_email: String,
        user_location: String,
        user_account_type: String,
    }

    impl Display for Info {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "{}",
                [
                    format!("user_name:         {}", self.user_name),
                    format!("user_format_short: {}", self.user_format_short),
                    format!("user_expiration:   {}", self.user_expiration),
                    format!("user_private:      {}", self.user_private),
                    format!("user_website:      {}", self.user_website),
                    format!("user_email:        {}", self.user_email),
                    format!("user_location:     {}", self.user_location),
                    format!("user_account_type: {}", self.user_account_type)
                ]
                .join("\n")
            )
        }
    }
}
