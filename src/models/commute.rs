use postgres::rows::Row;
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use time::{Timespec, at};

pub struct Commute {
    pub id: i32,
    pub user_id: i32,
    pub departed_at: Option<Timespec>,
    pub arrived_at: Option<Timespec>,
    pub created_at: Option<Timespec>,
    pub updated_at: Option<Timespec>,
}

impl Commute {
    pub fn from_record(row: &Row) -> Commute {
        Commute {
            id: row.get("id"),
            user_id: row.get("user_id"),
            departed_at: row.get("departed_at"),
            arrived_at: row.get("arrived_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl ToJson for Commute {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("id".to_string(), self.id.to_json());
        map.insert("user_id".to_string(), self.user_id.to_json());
        map.insert("departed_at".to_string(), timespec_to_json(self.departed_at.unwrap()));
        map.insert("arrived_at".to_string(), timespec_to_json(self.arrived_at.unwrap()));
        map.insert("created_at".to_string(), timespec_to_json(self.created_at.unwrap()));
        map.insert("updated_at".to_string(), timespec_to_json(self.updated_at.unwrap()));

        Json::Object(map)
    }
}

fn timespec_to_json(timespec: Timespec) -> Json {
    at(timespec).to_utc().rfc3339().to_string().to_json()
}
