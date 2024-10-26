use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{assets, liabilities, equity};

// For the assets table
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "assets"]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "assets"]
pub struct NewAsset {
    pub name: String,
    pub debit: f64,
    pub credit: f64,
}


// For the liabilities table
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "liabilities"]
pub struct Liability {
    pub id: i64,
    pub name: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "liabilities"]
pub struct NewLiability {
    pub name: String,
    pub debit: f64,
    pub credit: f64,
}

// For the equity table
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "equity"]
pub struct Equity {
    pub id: i64,
    pub name: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "equity"]
pub struct NewEquity {
    pub name: String,
    pub debit: f64,
    pub credit: f64,
}