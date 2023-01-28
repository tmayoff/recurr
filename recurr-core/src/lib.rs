use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Balances {
    pub available: Option<f64>,
    pub current: Option<f64>,
    pub limit: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub last_updated_datetime: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum AccountType {
    investment,
    credit,
    depository,
    loan,
    broakerage,
    other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Account {
    pub account_id: String,
    pub balances: Balances,
    pub mask: Option<String>,
    pub name: String,
    pub official_name: Option<String>,

    #[serde(rename = "type")]
    pub account_type: String,
    pub subtype: String,
    // verification_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    item_id: String,
    instituion_id: Option<String>,

    available_products: Vec<String>,
    products: Vec<String>,
}
