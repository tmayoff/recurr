use super::Msg;
use crate::{
    commands::{
        self, invokeItemPublicTokenExchange,
        link::{link_token_create, LinkFailure, LinkSuccess},
    },
    context::SessionContext,
    supabase::Session,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::{
    function_component, html, platform::spawn_local, use_context, Callback, Html, Properties,
};

#[derive(Serialize, Debug)]
pub struct User {
    pub client_user_id: String,
}

#[derive(Serialize, Debug)]
pub struct LinkTokenCreateRequest {
    pub client_name: String,
    pub language: String,
    pub country_codes: Vec<String>,
    pub products: Vec<String>,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicTokenExchangeResponse {
    pub access_token: String,
    pub item_id: String,
    pub request_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Institution {
    name: String,
    institution_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Account {
    id: String,
    name: String,
    mask: Option<String>,
    // type: String,
    verification_status: Option<String>,
    class_type: Option<String>,
}

async fn item_public_token_exchange(
    anon_key: &str,
    public_token: &str,
) -> Result<PublicTokenExchangeResponse, JsValue> {
    let res = invokeItemPublicTokenExchange(anon_key, public_token).await?;
    let s = serde_wasm_bindgen::from_value::<PublicTokenExchangeResponse>(res)
        .expect("Failed to deserialize");
    Ok(s)
}

fn link_callback(session: Session, result: Result<LinkSuccess, LinkFailure>) {
    let link_status = result.expect("Failed to get link");
    log::info!("Trying to save access token");

    spawn_local(async move {
        let exchange_status =
            item_public_token_exchange(&session.auth_key, &link_status.public_token).await;
        if let Err(e) = exchange_status {
            log::error!("{:?}", e);
            return;
        }
        let exchange_status = exchange_status.ok().unwrap();

        let user_id = &session.user.id;
        let auth_token = &session.auth_key;

        let client = recurr_core::get_supbase_client();

        let body = serde_json::to_string(&recurr_core::SchemaAccessToken {
            id: 0,
            access_token: exchange_status.access_token.clone(),
            user_id: user_id.to_owned(),
            plaid_accounts: None,
        })
        .expect("Failed to serialize schema");

        let res = client
            .from("access_tokens")
            .auth(auth_token)
            .insert(&body)
            .execute()
            .await
            .map(|e| e.error_for_status());
        log::info!("Trying to save access token");
        if let Err(e) = res {
            log::error!("{:?}", e);
            return;
        }

        let access_token = exchange_status.access_token;
        let client = recurr_core::get_supbase_client();

        let res = client
            .from("access_tokens")
            .auth(auth_token)
            .select("*")
            .eq("access_token", access_token)
            .eq("user_id", user_id)
            .execute()
            .await
            .map(|e| e.error_for_status())
            .expect("Failed to get access token")
            .expect("Failed to get access token");

        let schemas: Vec<recurr_core::SchemaAccessToken> =
            res.json().await.expect("Failed to get response");
        let access_token_row = schemas
            .first()
            .expect("No access token associated for these accounts");

        for account in link_status.metadata.accounts {
            let body = serde_json::to_string(&recurr_core::SchemaPlaidAccount {
                user_id: user_id.to_owned(),
                account_id: account.id,
                access_token_id: access_token_row.id,
            })
            .expect("Failed to serialize");

            let res = client
                .from("plaid_accounts")
                .auth(auth_token)
                .insert(&body)
                .execute()
                .await
                .map(|e| e.error_for_status());

            if let Err(e) = res {
                log::error!("Failed to save account {:?}", e);
            }
        }
    });
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub on_link_change: Callback<super::Msg>,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let context = use_context::<SessionContext>().expect("No context");

    let cb = props.on_link_change.clone();
    let link = move |_| {
        let context = context.clone();
        let cb = cb.clone();
        spawn_local(async move {
            let context = context.clone();
            let session = context
                .supabase_session
                .clone()
                .expect("Needs session already");

            let response = link_token_create(&session.auth_key, &session.user.id, None).await;
            let link_token = match response {
                Ok(res) => res.link_token,
                Err(e) => {
                    log::error!("{:?}", e);
                    return;
                }
            };

            commands::link::start(link_token, move |res| {
                log::info!("Trying to save access token");
                link_callback(session.clone(), res);
                cb.emit(Msg::Refresh);
            });
        })
    };

    html! {
        <>
            <script src="https://cdn.plaid.com/link/v2/stable/link-initialize.js"></script>
            <button class="button is-success" type="button" onclick={link}>{"Link New Account"}</button>
        </>
    }
}
