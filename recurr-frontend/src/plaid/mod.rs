use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::{
    function_component, html,
    platform::{pinned::oneshot, spawn_local},
    use_context, Html,
};

use crate::{
    commands::{
        self, invokeItemPublicTokenExchange, invokeSaveAccessToken, invokeSavePlaidAccount,
        link::{link_token_create, LinkFailure, LinkSuccess},
    },
    context::SessionContext,
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

#[function_component(Link)]
pub fn link() -> Html {
    let context = use_context::<SessionContext>().expect("No context");

    let link = move |_| {
        let context = context.clone();
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

            let (tx, rx) = oneshot::channel::<Result<LinkSuccess, LinkFailure>>();

            let sender_mtx = Mutex::new(Some(tx));

            commands::link::start(link_token, move |response| {
                if let Some(tx) = sender_mtx.lock().unwrap().take() {
                    let _ = tx.send(response);
                }
            });

            let link_status = rx.await.expect("Failed to get link response");
            if let Err(e) = &link_status {
                log::error!("{:?}", e);
                return;
            }

            let link_status = link_status.expect("Checked for error");

            let exchange_status =
                item_public_token_exchange(&session.auth_key, &link_status.public_token).await;
            if let Err(e) = exchange_status {
                log::error!("{:?}", e);
                return;
            }
            let exchange_status = exchange_status.ok().unwrap();

            let user_id = &session.user.id;
            let auth_token = &session.auth_key;

            let res =
                invokeSaveAccessToken(auth_token, user_id, &exchange_status.access_token).await;
            if let Err(e) = res {
                log::error!("{:?}", e);
                return;
            }

            for account in link_status.metadata.accounts {
                let res = invokeSavePlaidAccount(
                    auth_token,
                    user_id,
                    &exchange_status.access_token,
                    &account.account_id,
                )
                .await;

                if let Err(e) = res {
                    log::error!("{:?}", e);
                    return;
                }
            }
        })
    };

    html! {
        <>
            <script src="https://cdn.plaid.com/link/v2/stable/link-initialize.js"></script>
            <button class="button is-success" type="button" onclick={link}>{"Link New Account"}</button>
        </>
    }
}
