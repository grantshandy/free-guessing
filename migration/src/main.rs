use std::{
    collections::HashSet,
    fs::{self, File},
    path::PathBuf,
};

use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    record::RowAccessor,
};
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)?;

    let files = fs::read_dir("../data/")?
        .filter_map(|d| d.ok())
        .map(|f| f.path())
        .filter(|p| p.is_file());

    let sql_db = Connection::open("../public/locations.sqlite")?;

    let mut countries = HashSet::new();

    sql_db.execute_batch(
        "CREATE TABLE IF NOT EXISTS country_codes (
            country_id INTEGER PRIMARY KEY AUTOINCREMENT,
            country_code TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS locations (
            latlng BLOB NOT NULL,
            country_id INTEGER NOT NULL,
            FOREIGN KEY (country_id) REFERENCES country_codes(country_id)
        );",
    )?;

    for file in files {
        insert_parquet(&boundaries, &mut countries, file, &sql_db)?;
    }

    sql_db.cache_flush()?;

    fs::write(
        "../src/countries.json",
        serde_json::to_string_pretty(&serde_json::json!({ "codes": countries }))?,
    )?;

    Ok(())
}

fn insert_parquet(
    boundaries: &CountryBoundaries,
    countries: &mut HashSet<String>,
    pq_file: PathBuf,
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let reader = SerializedFileReader::new(File::open(pq_file)?)?;
    for row in reader.get_row_iter(None)? {
        let row = row?;

        let latitude = row.get_string(2)?.parse::<f32>()?;
        let longitude = row.get_string(3)?.parse::<f32>()?;

        let Some(country_code) = boundaries
            .ids(LatLon::new(latitude.into(), longitude.into())?)
            .into_iter()
            .last()
        else {
            return Ok(());
        };

        countries.insert(country_code.to_string());

        let country_id: usize = conn
            .query_row(
                "SELECT country_id FROM country_codes WHERE country_code = ?1",
                rusqlite::params![country_code],
                |row| row.get(0),
            )
            .or_else(|_| {
                conn.execute(
                    "INSERT INTO country_codes (country_code) VALUES (?1)",
                    rusqlite::params![country_code],
                )?;
                conn.query_row(
                    "SELECT country_id FROM country_codes WHERE country_code = ?1",
                    rusqlite::params![country_code],
                    |row| row.get(0),
                )
            })?;

        let latlng = [latitude.to_le_bytes(), longitude.to_le_bytes()].concat();

        conn.execute(
            "INSERT INTO locations (latlng, country_id) VALUES (?1, ?2)",
            rusqlite::params![latlng, country_id],
        )?;
    }

    Ok(())
}
