extern crate postgres;
extern crate time;

use postgres::{Connection, SslMode};
use time::{Timespec, strftime, at};

struct Commute {
    id: i32,
    user_id: i32,
    departed_at: Option<Timespec>,
    arrived_at: Option<Timespec>,
    created_at: Option<Timespec>,
    updated_at: Option<Timespec>,
}

fn main() {
    let conn = Connection::connect(
        "postgres://chris@localhost/commute_tracker_development",
        &SslMode::None,
    ).unwrap();

    let stmt = conn.prepare("SELECT * FROM commutes").unwrap();

    for row in stmt.query(&[]).unwrap() {
        let commute = Commute {
            id: row.get("id"),
            user_id: row.get("user_id"),
            departed_at: row.get("departed_at"),
            arrived_at: row.get("arrived_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        println!(
            "user {} departed at {} and arrived at {}",
            commute.user_id,
            strftime("%m-%d-%y %H:%M:%S UTC", &at(commute.departed_at.unwrap())).unwrap(),
            strftime("%m-%d-%y %H:%M:%S UTC", &at(commute.arrived_at.unwrap())).unwrap(),
        );
    }
}
