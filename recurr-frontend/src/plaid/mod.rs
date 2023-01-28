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
        invokeItemPublicTokenExchange, invokeLinkTokenCreate, invokeSaveAccessToken,
        invokeSavePlaidAccount, linkStart,
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

impl LinkTokenCreateRequest {
    pub fn new(
        client_name: &str,
        language: &str,
        country_codes: Vec<String>,
        products: Vec<String>,
        user: User,
    ) -> Self {
        Self {
            client_name: client_name.to_string(),
            language: language.to_string(),
            country_codes: country_codes.to_vec(),
            products: products,
            user: user,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct LinkTokenCreateReponse {
    expiration: String,
    link_token: String,
    request_id: String,
}

pub async fn link_token_create(anon_key: &str) -> Result<LinkTokenCreateReponse, String> {
    let response = invokeLinkTokenCreate(anon_key).await;

    match response {
        Ok(response) => {
            let res = serde_wasm_bindgen::from_value::<LinkTokenCreateReponse>(response)
                .expect("Response not valid");
            Ok(res)
        }
        Err(e) => Err(e.as_string().unwrap()),
    }
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

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    institution: Option<Institution>,
    accounts: Vec<Account>,
    link_session_id: String,
    transfer_status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LinkSuccess {
    public_token: String,
    metadata: Metadata,
}
#[derive(Debug, Deserialize, Serialize)]
struct LinkFailure {
    err: String,
}

fn link_start(
    link_token: String,
    mut callback: impl FnMut(Result<LinkSuccess, LinkFailure>) + 'static,
) {
    linkStart(
        link_token,
        Closure::once_into_js(move |response: JsValue| {
            let s = serde_wasm_bindgen::from_value::<LinkSuccess>(response.clone());
            if let Ok(success) = s {
                callback(Ok(success));
                return;
            };

            let e = serde_wasm_bindgen::from_value::<LinkFailure>(response.clone());
            if let Ok(failure) = e {
                callback(Err(failure));
                return;
            };
        }),
    );
}

async fn item_public_token_exchange(
    anon_key: &str,
    public_token: &str,
) -> Result<PublicTokenExchangeResponse, JsValue> {
    let res = invokeItemPublicTokenExchange(anon_key, public_token).await;
    match res {
        Ok(s) => {
            let s = serde_wasm_bindgen::from_value::<PublicTokenExchangeResponse>(s)
                .expect("Failed to deserialize");
            Ok(s)
        }
        Err(e) => Err(e),
    }
}

#[function_component(Link)]
pub fn link() -> Html {
    let context = use_context::<SessionContext>().expect("No context");

    let link = move |_| {
        let context = context.clone();
        spawn_local(async move {
            let context = context.clone();
            let response = link_token_create(&context.anon_key).await;
            let link_token;
            match response {
                Ok(res) => {
                    link_token = res.link_token;
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    return;
                }
            }

            let (tx, rx) = oneshot::channel::<Result<LinkSuccess, LinkFailure>>();

            let sender_mtx = Mutex::new(Some(tx));

            link_start(link_token, move |response| {
                if let Some(tx) = sender_mtx.lock().unwrap().take() {
                    let _ = tx.send(response);
                }
            });

            if let Ok(response) = rx.await {
                match response {
                    Ok(success) => {
                        let res =
                            item_public_token_exchange(&context.anon_key, &success.public_token)
                                .await;

                        match res {
                            Ok(s) => {
                                let context = context.clone();
                                if let Some(session) = &context.supabase_session {
                                    let user_id = &session.user.id;
                                    let auth_token = &session.access_token;

                                    let res =
                                        invokeSaveAccessToken(auth_token, user_id, &s.access_token)
                                            .await;

                                    if let Err(e) = res {
                                        log::error!("{:?}", e);
                                        return;
                                    }

                                    for account in success.metadata.accounts {
                                        let res = invokeSavePlaidAccount(
                                            auth_token,
                                            user_id,
                                            &s.access_token,
                                            &account.id,
                                        )
                                        .await;
                                        if let Err(e) = res {
                                            log::error!("{:?}", e);
                                            return;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                log::info!("{:?}", e);
                            }
                        }
                    }
                    Err(error) => log::error!("{:?}", error),
                }
            };
        })
    };

    html! {
        <>
            <script src="https://cdn.plaid.com/link/v2/stable/link-initialize.js"></script>
            <button class="button is-success" type="button" onclick={link}>{"Link New Account"}</button>
        </>
    }
}
