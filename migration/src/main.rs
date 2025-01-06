use std::{env, fs::File};

use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    record::RowAccessor,
};
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)?;
    let pq_file = File::open(env::args().nth(1).unwrap())?;
    let sql_db = Connection::open("./panos.db3")?;

    sql_db.execute(
        "CREATE TABLE IF NOT EXISTS locations (
            latitude REAL NOT NULL,
            longitude REAL NOT NULL,
            country TEXT NOT NULL
        )",
        (),
    )?;

    let reader = SerializedFileReader::new(pq_file)?;
    for (i, row) in reader.get_row_iter(None)?.enumerate() {
        let row = row?;

        if i % 1000 == 0 {
            println!("{i}");
        }

        let latitude = row.get_string(2)?.parse::<f64>()?;
        let longitude = row.get_string(3)?.parse::<f64>()?;

        let country = boundaries
            .ids(LatLon::new(latitude, longitude)?)
            .into_iter()
            .last()
            .unwrap_or("??");

        sql_db.execute(
            "INSERT INTO locations(latitude, longitude, country) VALUES (?1, ?2, ?3)",
            rusqlite::params![latitude, longitude, country],
        )?;
    }

    sql_db.cache_flush()?;

    Ok(())
}
