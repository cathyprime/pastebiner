pub(crate) mod styles {
    use ansi_term::{Color, Style};

    pub fn missing() -> Style {
        Color::Red.bold()
    }

    pub fn date() -> Color {
        Color::Green
    }

    pub fn privacy() -> Style {
        Color::Yellow.underline()
    }

    pub fn key() -> Color {
        Color::Purple
    }
}

pub(crate) mod file_size {
    use serde::Deserialize;
    use std::fmt::Display;

    #[derive(Debug)]
    pub(crate) struct Size {
        bytes: f64,
        suffix: Suffix,
    }

    impl Size {
        pub(crate) fn new(bytes: i32) -> Size {
            Self::calc_prefix(bytes as f64, Suffix::None)
        }

        fn calc_prefix(bytes: f64, prefix: Suffix) -> Size {
            if bytes >= 1024.0 {
                Self::calc_prefix(bytes / 1024.0, prefix.up())
            } else {
                Size {
                    bytes,
                    suffix: prefix,
                }
            }
        }
    }

    impl Display for Size {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.suffix {
                Suffix::None => write!(f, "{}{}", self.bytes, self.suffix),
                _ => write!(f, "{:.2}{}", self.bytes, self.suffix),
            }
        }
    }

    #[derive(Debug)]
    pub(crate) enum Suffix {
        None,
        Kilo,
        Mega,
        Giga,
        Tera,
    }

    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let bytes = i32::deserialize(deserializer)?;
            Ok(Size::new(bytes))
        }
    }

    impl Suffix {
        pub(crate) fn up(&self) -> Self {
            match self {
                Suffix::None => Suffix::Kilo,
                Suffix::Kilo => Suffix::Mega,
                Suffix::Mega => Suffix::Giga,
                Suffix::Giga => Suffix::Tera,
                Suffix::Tera => unimplemented!(),
            }
        }
    }

    impl Display for Suffix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Suffix::None => "B",
                    Suffix::Kilo => "KB",
                    Suffix::Mega => "MB",
                    Suffix::Giga => "GB",
                    Suffix::Tera => "TB",
                }
            )
        }
    }
}

pub(crate) mod private {
    use serde::Deserialize;
    use std::fmt::Display;

    #[derive(Debug, Clone)]
    pub(crate) enum Privacy {
        Public,
        Unlisted,
        Private,
    }

    impl<'de> Deserialize<'de> for Privacy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Privacy::try_from(s).map_err(serde::de::Error::custom)
        }
    }

    impl TryFrom<String> for Privacy {
        type Error = String;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.as_str() {
                "0" | "public" => Ok(Self::Public),
                "1" | "unlisted" => Ok(Self::Unlisted),
                "2" | "private" => Ok(Self::Private),
                _ => Err("value out of range".to_string()),
            }
        }
    }

    impl Privacy {
        pub(crate) fn form_ready(&self) -> String {
            match self {
                Privacy::Public   => "0".to_string(),
                Privacy::Unlisted => "1".to_string(),
                Privacy::Private  => "2".to_string(),
            }
        }
    }

    impl Display for Privacy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Privacy::Public => "Public",
                    Privacy::Unlisted => "Unlisted",
                    Privacy::Private => "Private",
                }
            )
        }
    }
}

pub(crate) mod string_to_datetime {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use std::time::{Duration, UNIX_EPOCH};

    pub(crate) fn string_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let duration = Duration::from_secs(s.parse::<u64>().map_err(serde::de::Error::custom)?);
        let datetime = DateTime::<Utc>::from(UNIX_EPOCH + duration);
        Ok(datetime)
    }
}

pub(crate) mod website {
    use super::styles::missing;

    #[derive(serde::Deserialize, Debug)]
    pub(crate) struct Website {
        url: Option<url::Url>,
    }

    impl std::fmt::Display for Website {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match &self.url {
                    Some(v) => v.to_string(),
                    None => missing().paint("<No website>").to_string(),
                }
            )
        }
    }
}
