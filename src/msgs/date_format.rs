pub mod date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";
    const FORMAT2: &'static str = "%Y-%m-%dT%H:%M:%S.%f%z";
    const FORMAT3: &'static str = "%Y-%m-%dT%H:%M:%S%:z";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT3));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut dt = NaiveDateTime::parse_from_str(&s, FORMAT);
        if dt.is_err() {
            dt = NaiveDateTime::parse_from_str(&s, FORMAT2);
        }
        if dt.is_err() {
            dt = NaiveDateTime::parse_from_str(&s, FORMAT3);   
        }
        let dt = dt.map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}