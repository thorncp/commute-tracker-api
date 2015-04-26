extern crate postgres;
extern crate time;

use postgres::{Connection, SslMode};
use time::{Timespec, strftime, at};

pub struct Commute {
    pub id: i32,
    pub user_id: i32,
    pub departed_at: Option<Timespec>,
    pub arrived_at: Option<Timespec>,
    pub created_at: Option<Timespec>,
    pub updated_at: Option<Timespec>,
}

pub struct CommutePresenter<'s> {
    commute: &'s Commute,
}

impl<'s> CommutePresenter<'s> {
    pub fn arrived_at(&self) -> String {
        self.format_time(self.commute.arrived_at)
    }

    pub fn departed_at(&self) -> String {
        self.format_time(self.commute.departed_at)
    }

    fn format_time(&self, time: Option<Timespec>) -> String {
        match time {
            // TODO: there's probably a better way to do this
            Some(time) => strftime("%m-%d-%y %H:%M:%S UTC", &at(time)).unwrap(),
            None => "unknown".to_string(),
        }
    }
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

        let presenter = CommutePresenter { commute: &commute };

        println!(
            "user {} departed at {} and arrived at {}",
            commute.user_id,
            presenter.departed_at(),
            presenter.arrived_at(),
        );
    }
}
