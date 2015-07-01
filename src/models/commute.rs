use postgres::rows::Row;
use rustc_serialize::Decoder;
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use super::timestamp::Timestamp;

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Commute {
    pub id: Option<i32>,
    pub user_id: i32,
    pub departed_at: Timestamp,
    pub arrived_at: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Commute {
    pub fn from_record(row: &Row) -> Commute {
        Commute {
            id: row.get("id"),
            user_id: row.get("user_id"),
            departed_at: Timestamp::Some(row.get("departed_at")),
            arrived_at: Timestamp::Some(row.get("arrived_at")),
            created_at: Timestamp::Some(row.get("created_at")),
            updated_at: Timestamp::Some(row.get("updated_at")),
        }
    }
}

// TODO: look at the serde crate as an alternative to rustc_serialize
impl ToJson for Commute {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("id".to_string(), self.id.to_json());
        map.insert("user_id".to_string(), self.user_id.to_json());
        map.insert("departed_at".to_string(), self.departed_at.to_json());
        map.insert("arrived_at".to_string(), self.arrived_at.to_json());
        map.insert("created_at".to_string(), self.created_at.to_json());
        map.insert("updated_at".to_string(), self.updated_at.to_json());

        Json::Object(map)
    }
}
