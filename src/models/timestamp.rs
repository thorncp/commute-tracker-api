use rustc_serialize::{Decoder, Decodable};
use rustc_serialize::json::{ToJson, Json};
use time::{Timespec, strptime, at};

#[derive(Debug)]
pub enum Timestamp {
    Some(Timespec),
    None,
}

impl Decodable for Timestamp {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_str().
            and_then(|s| strptime(&s, "%Y-%m-%dT%H:%M:%SZ").map_err(|_| decoder.error("ignored"))).
            and_then(|t| Ok(Timestamp::Some(t.to_timespec()))).
            or_else(|_| Ok(Timestamp::None))
    }
}

impl ToJson for Timestamp {
    fn to_json(&self) -> Json {
        match self {
            &Timestamp::Some(timespec) => at(timespec).to_utc().rfc3339().to_string().to_json(),
            &Timestamp::None => Json::Null
        }
    }
}
