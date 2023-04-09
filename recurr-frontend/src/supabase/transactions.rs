use recurr_core::Transaction;

pub async fn get_transactions_paged(
    auth_key: &str,
    page: u64,
    per_page: u64,
) -> Result<(u64, Vec<Transaction>), recurr_core::Error> {
    let db_client = recurr_core::get_supbase_client();

    let res = db_client
        .from("transactions")
        .auth(&auth_key)
        .select("*")
        .order("date.desc")
        .exact_count()
        .range(
            (page * per_page) as usize,
            (page * per_page + per_page) as usize,
        )
        .execute()
        .await?
        .error_for_status()?;

    let total_transactions = res
        .headers()
        .get("content-range")
        .expect("Failed to get total count")
        .to_str()
        .unwrap()
        .split('/')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let transactions: Vec<Transaction> = res.json().await?;
    Ok((total_transactions, transactions))
}
