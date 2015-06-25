#[macro_use] extern crate nickel;
extern crate postgres;
extern crate rustc_serialize;
extern crate time;

use nickel::{Nickel, HttpRouter};
use rustc_serialize::json::ToJson;
use postgres::{Connection, SslMode};
use models::Commute;

mod models;

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

    router.get("/commutes", middleware! { |_request|
        let conn = connection();
        let stmt = conn.prepare("SELECT * FROM commutes").unwrap();

        let commutes = stmt.query(&[]).unwrap().iter().map( |row|
            Commute::from_record(&row)
        ).collect::<Vec<Commute>>();

        commutes.to_json()
    });

    router.get("/commutes/:id", middleware! { |request|
        let conn = connection();
        let commute_id = request.param("id").parse::<i32>().unwrap();
        let stmt = conn.prepare("SELECT * FROM commutes WHERE id = $1 LIMIT 1").unwrap();
        let results = stmt.query(&[&commute_id]).unwrap();
        // TODO: handle no records returned
        let row = results.iter().next().unwrap();
        let commute = Commute::from_record(&row);

        commute.to_json()
    });

    server.utilize(router);
    server.listen("127.0.0.1:6767");
}
