#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use crate::models::{Asset, Equity, Liability, NewAsset, NewEquity, NewLiability};
use crate::schema::{assets, equity, liabilities};
use diesel::prelude::*; // Bring Diesel's query-related traits into scope
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_sync_db_pools::{database, diesel::MysqlConnection};
use std::collections::HashMap;

mod models;
mod schema;

#[database("mysql_db")]
struct DbConn(MysqlConnection);

// Struct to define JSON input for Asset, Liability, and Equity
#[derive(Serialize, Deserialize)]
struct InputData {
    name: String,
    debit: f64,
    credit: f64,
}

#[post("/assets", data = "<new_asset>")]
async fn create_asset(conn: DbConn, new_asset: Json<NewAsset>) -> Json<Asset> {
    conn.run(move |c| {
        // Dereference the Json wrapper to get the NewAsset struct
        diesel::insert_into(assets::table)
            .values(&*new_asset) // Dereference with `*` to get the inner NewAsset
            .execute(c)
            .unwrap();

        // Query the newly inserted asset
        assets::table
            .order(assets::id.desc())
            .first::<Asset>(c)
            .unwrap()
    })
    .await
    .into()
}

// GET route to retrieve all assets
#[get("/assets")]
async fn get_assets(conn: DbConn) -> Json<Vec<Asset>> {
    conn.run(|c| assets::table.load::<Asset>(c).unwrap())
        .await
        .into()
}

// GET a specific asset by id
#[get("/assets/<id>")]
async fn get_asset_by_id(conn: DbConn, id: i64) -> Option<Json<Asset>> {
    conn.run(move |c| {
        assets::table
            .filter(assets::id.eq(id))
            .first::<Asset>(c)
            .optional()
            .unwrap()
    })
    .await
    .map(Json)
}

// DELETE asset by id
#[delete("/assets/<id>")]
async fn delete_asset(conn: DbConn, id: i64) -> Json<String> {
    conn.run(move |c| {
        let result = diesel::delete(assets::table.filter(assets::id.eq(id))).execute(c);
        match result {
            Ok(_) => Json(format!("Asset with ID {} deleted", id)),
            Err(_) => Json(format!("Failed to delete asset with ID {}", id)),
        }
    })
    .await
}

#[post("/liabilities", data = "<new_liability>")]
async fn create_liability(conn: DbConn, new_liability: Json<NewLiability>) -> Json<Liability> {
    conn.run(move |c| {
        diesel::insert_into(liabilities::table)
            .values(&*new_liability) // Dereference Json<NewLiability>
            .execute(c)
            .unwrap();

        liabilities::table
            .order(liabilities::id.desc())
            .first::<Liability>(c)
            .unwrap()
    })
    .await
    .into()
}

// GET route to retrieve all liabilities
#[get("/liabilities")]
async fn get_liabilities(conn: DbConn) -> Json<Vec<Liability>> {
    conn.run(|c| liabilities::table.load::<Liability>(c).unwrap())
        .await
        .into()
}

// GET a specific liability by id
#[get("/liabilities/<id>")]
async fn get_liability_by_id(conn: DbConn, id: i64) -> Option<Json<Liability>> {
    conn.run(move |c| {
        liabilities::table
            .filter(liabilities::id.eq(id))
            .first::<Liability>(c)
            .optional()
            .unwrap()
    })
    .await
    .map(Json)
}

// DELETE liability by id
#[delete("/liabilities/<id>")]
async fn delete_liability(conn: DbConn, id: i64) -> Json<String> {
    conn.run(move |c| {
        let result = diesel::delete(liabilities::table.filter(liabilities::id.eq(id))).execute(c);
        match result {
            Ok(_) => Json(format!("Liability with ID {} deleted", id)),
            Err(_) => Json(format!("Failed to delete liability with ID {}", id)),
        }
    })
    .await
}

#[post("/equity", data = "<new_equity>")]
async fn create_equity(conn: DbConn, new_equity: Json<NewEquity>) -> Json<Equity> {
    conn.run(move |c| {
        diesel::insert_into(equity::table)
            .values(&*new_equity) // Dereference Json<NewEquity>
            .execute(c)
            .unwrap();

        equity::table
            .order(equity::id.desc())
            .first::<Equity>(c)
            .unwrap()
    })
    .await
    .into()
}

// GET route to retrieve all equity entries
#[get("/equity")]
async fn get_equity(conn: DbConn) -> Json<Vec<Equity>> {
    conn.run(|c| equity::table.load::<Equity>(c).unwrap())
        .await
        .into()
}

// GET a specific equity entry by id
#[get("/equity/<id>")]
async fn get_equity_by_id(conn: DbConn, id: i64) -> Option<Json<Equity>> {
    conn.run(move |c| {
        equity::table
            .filter(equity::id.eq(id))
            .first::<Equity>(c)
            .optional()
            .unwrap()
    })
    .await
    .map(Json)
}

// DELETE equity by id
#[delete("/equity/<id>")]
async fn delete_equity(conn: DbConn, id: i64) -> Json<String> {
    conn.run(move |c| {
        let result = diesel::delete(equity::table.filter(equity::id.eq(id))).execute(c);
        match result {
            Ok(_) => Json(format!("Equity with ID {} deleted", id)),
            Err(_) => Json(format!("Failed to delete equity with ID {}", id)),
        }
    })
    .await
}

// Rocket launch configuration
#[launch]
fn rocket() -> _ {
    // Set up CORS options
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000", // Add your frontend URL here
        "http://localhost:8000",
    ]);

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .to_cors()
        .expect("Error creating CORS fairing");

    rocket::build()
        .attach(DbConn::fairing()) // Attaching the MySQL database connection
        //.attach(cors) // Attach CORS directly as a fairing
        .mount(
            "/",
            routes![
                get_assets,          // Getting all assets
                get_asset_by_id,     // Get a specific asset entry by ID
                create_asset,        // Creating a new asset
                delete_asset,        // Route to delete an asset
                get_liabilities,     // Getting all liabilities
                get_liability_by_id, // Get a specific liability entry by ID
                create_liability,    // Creating a new liability
                delete_liability,    // Route to delete a liability
                get_equity,          // Getting all equity entries
                get_equity_by_id,    // Get a specific equity entry by ID
                create_equity,       // Creating a new equity entry
                delete_equity        // Route to delete equity
            ],
        )
}
