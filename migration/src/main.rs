use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    path::PathBuf,
};

use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    record::RowAccessor,
};
use rusqlite::Connection;

const PARQUET_DIR: &str = "../data/";
const LOCATIONS_DB: &str = "../public/locations.sqlite";
const COUNTRIES_JSON: &str = "../src/countries.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)?;

    let pq_files = fs::read_dir(PARQUET_DIR)?
        .filter_map(|d| d.ok())
        .map(|f| f.path())
        .filter(|p| p.is_file());

    let _ = fs::remove_file(LOCATIONS_DB);
    let sql_db = Connection::open(LOCATIONS_DB)?;

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

    for file in pq_files {
        println!("processing {file:?}");
        insert_parquet(&boundaries, &mut countries, file, &sql_db)?;
    }

    sql_db.cache_flush()?;

    write_countries(countries)?;

    Ok(())
}

fn write_countries(needed: HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let all_countries: serde_json::Value =
        serde_json::from_str(include_str!("all_countries.json"))?;

    fs::write(
        COUNTRIES_JSON,
        serde_json::to_vec_pretty::<HashMap<String, Vec<f64>>>(&HashMap::from_iter(
            needed.into_iter().map(|d| {
                (
                    d.clone(),
                    all_countries[d][1]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_f64().unwrap())
                        .collect::<Vec<f64>>(),
                )
            }),
        ))?,
    )?;

    Ok(())
}

fn insert_parquet(
    boundaries: &CountryBoundaries,
    countries: &mut HashSet<String>,
    pq_file: PathBuf,
    sql_db: &Connection,
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

        let country_id: usize = sql_db
            .query_row(
                "SELECT country_id FROM country_codes WHERE country_code = ?1",
                rusqlite::params![country_code],
                |row| row.get(0),
            )
            .or_else(|_| {
                sql_db.execute(
                    "INSERT INTO country_codes (country_code) VALUES (?1)",
                    rusqlite::params![country_code],
                )?;
                sql_db.query_row(
                    "SELECT country_id FROM country_codes WHERE country_code = ?1",
                    rusqlite::params![country_code],
                    |row| row.get(0),
                )
            })?;

        let latlng = [latitude.to_le_bytes(), longitude.to_le_bytes()].concat();

        sql_db.execute(
            "INSERT INTO locations (latlng, country_id) VALUES (?1, ?2)",
            rusqlite::params![latlng, country_id],
        )?;
    }

    Ok(())
}
