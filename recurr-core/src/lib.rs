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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Category {
    pub category_id: String,
    pub group: String,
    pub hierarchy: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Institution {
    pub institution_id: String,
    pub name: String,
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

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transactions {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
    pub total_transactions: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionOption {
    pub account_ids: Vec<String>,
    pub count: Option<i32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    pub name: String,
    pub amount: f64,
    pub category_id: Option<String>,
    pub category: Vec<String>,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub item_id: String,
    pub institution_id: Option<String>,

    pub available_products: Vec<String>,
    pub products: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SchemaBudget {
    pub user_id: String,
    pub category: String,
    pub max: f64,
}

impl std::hash::Hash for SchemaBudget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
        self.category.hash(state);
    }
}

impl std::cmp::Eq for SchemaBudget {}

impl SchemaBudget {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaAccessToken {
    #[serde(skip_serializing)]
    pub id: i32,
    pub access_token: String,
    pub user_id: String,

    #[serde(skip_serializing)]
    pub plaid_accounts: Option<Vec<SchemaPlaidAccount>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaPlaidAccount {
    pub user_id: String,
    pub account_id: String,
    pub access_token_id: i32,
}
