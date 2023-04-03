use recurr_core::{Error, SchemaAccessToken};

#[tauri::command]
pub async fn get_access_tokens(
    auth_token: &str,
    user_id: &str,
) -> Result<SchemaAccessToken, Error> {
    let client = super::get_supbase_client()?;

    let res = client
        .from("access_tokens")
        .auth(auth_token)
        .select("*")
        .eq("user_id", user_id)
        .execute()
        .await
        .map(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    let json = res
        .json()
        .await
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;
    let schemas: Vec<SchemaAccessToken> = json;

    let schema = schemas.first();
    match schema {
        Some(schema) => Ok(schema.clone()),
        None => Err(Error::Query("Nothing returned from query".to_string())),
    }
}

#[tauri::command]
pub async fn get_access_token(
    auth_token: &str,
    user_id: &str,
    access_token: &str,
) -> Result<SchemaAccessToken, super::Error> {
    let client = super::get_supbase_client()?;

    let res = client
        .from("access_tokens")
        .auth(auth_token)
        .select("*")
        .eq("access_token", access_token)
        .eq("user_id", user_id)
        .execute()
        .await
        .map(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    let json = res
        .json()
        .await
        .map_err(|e| recurr_core::Error::Request(e.to_string()))?;

    let schemas: Vec<SchemaAccessToken> = json;
    let schema = schemas.first();
    match schema {
        Some(schema) => Ok(schema.clone()),
        None => Err(Error::Query("Nothing returned from query".to_string())),
    }
}

#[tauri::command]
pub async fn save_access_token(
    auth_token: &str,
    user_id: &str,
    access_token: &str,
) -> Result<(), super::Error> {
    let client = super::get_supbase_client()?;

    let body = serde_json::to_string(&SchemaAccessToken {
        id: 0,
        access_token: access_token.to_owned(),
        user_id: user_id.to_owned(),
        plaid_accounts: None,
    })
    .expect("Failed to serialize schema");

    let _ = client
        .from("access_tokens")
        .auth(auth_token)
        .insert(&body)
        .execute()
        .await
        .map(|e| e.error_for_status())
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?
        .map_err(|e| recurr_core::Error::Other(e.to_string()))?;

    Ok(())
}
