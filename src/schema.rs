// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        debit -> Double,
        credit -> Double,
    }
}

diesel::table! {
    equity (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        debit -> Double,
        credit -> Double,
    }
}

diesel::table! {
    liabilities (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        debit -> Double,
        credit -> Double,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    assets,
    equity,
    liabilities,
);
