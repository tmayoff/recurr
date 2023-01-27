use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Balances {
    available: Option<f64>,
    current: Option<f64>,
    limit: Option<f64>,
    iso_currency_code: Option<String>,
    unofficial_currency_code: Option<String>,
    last_updated_datetime: Option<String>,
}

#[derive(Serialize, Deserialize)]
enum AccountType {
    investment,
    credit,
    depository,
    loan,
    broakerage,
    other,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    account_id: String,
    balances: Balances,
    mask: Option<String>,
    name: String,
    official_name: Option<String>,
    #[serde(rename = "type")]
    account_type: String,
    verification_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    item_id: String,
    instituion_id: Option<String>,

    available_products: Vec<String>,
    products: Vec<String>,
}
