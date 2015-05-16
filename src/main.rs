extern crate nickel;
extern crate postgres;
extern crate rustc_serialize;
extern crate time;
#[macro_use] extern crate nickel_macros;

use std::collections::BTreeMap;
use nickel::{Nickel, HttpRouter};
use rustc_serialize::json::{Json, ToJson};
use postgres::{Connection, SslMode};
use time::{Timespec, at};

pub struct Commute {
    pub id: i32,
    pub user_id: i32,
    pub departed_at: Option<Timespec>,
    pub arrived_at: Option<Timespec>,
    pub created_at: Option<Timespec>,
    pub updated_at: Option<Timespec>,
}

fn timespec_to_json(timespec: Timespec) -> Json {
    time::at(timespec).to_utc().rfc3339().to_string().to_json()
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

// TODO: keep a connection (or pool) open instead of reconnecting every time
fn connection() -> Connection {
    Connection::connect(
        "postgres://chris@localhost/commute_tracker_development",
        &SslMode::None,
    ).unwrap()
}

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/commutes", middleware! { |request|
        let conn = connection();
        let stmt = conn.prepare("SELECT * FROM commutes").unwrap();

        let commutes = stmt.query(&[]).unwrap().iter().map( |row|
            Commute {
                id: row.get("id"),
                user_id: row.get("user_id"),
                departed_at: row.get("departed_at"),
                arrived_at: row.get("arrived_at"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        ).collect::<Vec<Commute>>();

        commutes.to_json()
    });

    router.get("/commutes/:id", middleware! { |request|
        let conn = connection();
        let commute_id = request.param("id").parse::<i32>().unwrap();
        let stmt = conn.prepare("SELECT * FROM commutes WHERE id = $1 LIMIT 1").unwrap();
        let results = stmt.query(&[&commute_id]).unwrap();
        let row = results.iter().next().unwrap();

        let commute = Commute {
            id: row.get("id"),
            user_id: row.get("user_id"),
            departed_at: row.get("departed_at"),
            arrived_at: row.get("arrived_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        commute.to_json()
    });

    server.utilize(router);
    server.listen("127.0.0.1:6767");
}
