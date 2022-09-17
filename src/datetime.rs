use chrono::NaiveDate;

pub struct EODHDDate(pub i32, pub u32, pub u32);

impl From<EODHDDate> for NaiveDate {
    fn from(ed: EODHDDate) -> Self {
        NaiveDate::from_ymd(ed.0, ed.1, ed.2)
    }
}

pub enum EODHDInterval {
    Minute,
    Minute5,
    Hour,
}

impl ToString for EODHDInterval {
    fn to_string(&self) -> String {
        match self {
            EODHDInterval::Hour => "1h",
            EODHDInterval::Minute => "1m",
            EODHDInterval::Minute5 => "5m",
        }
        .to_string()
    }
}

pub mod eodhd_serde_opt_date {
    use chrono::NaiveDate;
    use log::error;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";
    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if date.is_some() {
            let s = format!("{}", date.unwrap().format(FORMAT));
            return serializer.serialize_str(&s);
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parsed = NaiveDate::parse_from_str(&s, FORMAT);
        match parsed {
            Ok(parsed) => {
                Ok(Some(parsed))
            },
            Err(e) => {
                error!("error parsing serde date {:?}", e);
                Err(serde::de::Error::custom("faulty eodhd date"))
            }
        }
    }
}

pub mod eodhd_serde_datetime {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod eodhd_serde_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";
    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
