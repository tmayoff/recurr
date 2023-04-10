mod link;

use crate::{commands, context::Session};
use futures::future;
use link::Link;
use recurr_core::{get_supbase_client, Account, Institution, SchemaAccessToken};
use serde::Deserialize;
use std::collections::HashMap;
use yew::{
    function_component, html, platform::spawn_local, Callback, Component, Html, Properties,
    UseReducerHandle,
};
use yew_hooks::use_bool_toggle;

pub struct AccountsView {
    accounts: HashMap<Institution, Vec<Account>>,
    error: String,
}

pub enum Msg {
    GetAccounts,
    GotAccounts(HashMap<Institution, Vec<Account>>),

    Error(String),

    Refresh,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub context: UseReducerHandle<Session>,
}

impl Component for AccountsView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetAccounts);

        Self {
            accounts: HashMap::new(),
            error: String::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let auth_key = ctx
            .props()
            .context
            .clone()
            .supabase_session
            .clone()
            .unwrap()
            .auth_key;

        let user_id = ctx
            .props()
            .context
            .clone()
            .supabase_session
            .clone()
            .unwrap()
            .user
            .id;

        let link_cb = ctx.link().callback(|msg| msg);

        html! {
            <div>
                <div class="is-flex is-flex-direction-row is-justify-content-space-around is-align-items-center">
                    <h1 class="is-size-3">{"All Accounts"}</h1>
                    <Link on_link_change={link_cb}/>
                </div>
                <div class="is-flex p-2">
                if self.accounts.is_empty() {
                    <progress class="progress is-small is-primary" max="100">{"15%"}</progress>
                } else {
                    {
                        self.accounts.clone().into_iter().map(|a| {
                            html!{<AccountItem account={a} auth_key={auth_key.clone()} user_id={user_id.clone()} />}
                        }).collect::<Html>()
                    }
                }
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetAccounts => {
                let supabase_session = ctx.props().context.supabase_session.clone().unwrap();

                let auth_key = supabase_session.auth_key;
                // let user_id = supabase_session.user.id;

                ctx.link().send_future(async move {
                    let client = get_supbase_client();
                    let res = client
                        .from("access_tokens")
                        .auth(&auth_key)
                        .select("*,plaid_accounts(*)")
                        .execute()
                        .await;

                    if let Err(e) = res {
                        return Msg::Error(e.to_string());
                    }

                    let rows: Vec<SchemaAccessToken> =
                        res.unwrap().json().await.expect("Failed to deserialize");

                    let mut futures = Vec::new();
                    for row in rows {
                        if row.plaid_accounts.is_none() {
                            continue;
                        }

                        futures.push(get_accounts(&auth_key, row.clone()));
                    }

                    let results = future::join_all(futures).await;

                    let mut all_accounts = HashMap::new();
                    for res in results {
                        if let Err(e) = &res {
                            if let recurr_core::Error::Plaid(e) = e {
                                if &e.error_code == "ITEM_LOGIN_REQUIRED" {
                                    log::error!("Needs login");

                                    //         let link_token = commands::link::link_token_create(
                                    //             &auth_key,
                                    //             &user_id,
                                    //             Some(res.to_owned()),
                                    //         )
                                    //         .await
                                    //         .expect("failed to get link token");

                                    //         let link_token = link_token.link_token;
                                    //         let (tx, rx) = oneshot::channel::<()>();
                                    //         let sender_mtx = Mutex::new(Some(tx));

                                    //         commands::link::start(link_token, move |_| {
                                    //             if let Some(tx) = sender_mtx.lock().unwrap().take() {
                                    //                 let _ = tx.send(());
                                    //             }
                                    //         });

                                    //         rx.await.expect("Failed to update token");

                                    //         return Msg::Refresh;
                                }
                            } else {
                                return Msg::Error(e.to_string());
                            }
                        }

                        let res = res.unwrap();
                        all_accounts.insert(res.0, res.1);
                    }

                    Msg::GotAccounts(all_accounts)
                })
            }
            Msg::GotAccounts(a) => self.accounts = a,
            Msg::Error(e) => self.error = e,
            Msg::Refresh => (),
        }

        true
    }
}

async fn get_accounts(
    auth_key: &str,
    access_token_row: SchemaAccessToken,
) -> Result<(Institution, Vec<Account>), recurr_core::Error> {
    let access_token = access_token_row.access_token.as_ref();

    let account_ids = access_token_row
        .plaid_accounts
        .as_ref()
        .unwrap()
        .iter()
        .map(|a| a.account_id.clone())
        .collect();

    let res = commands::get_accounts(auth_key, access_token, account_ids).await?;

    let institution_id = res.0.institution_id.unwrap();
    let insitution = commands::get_institution(auth_key, Some(institution_id))
        .await
        .unwrap();

    Ok((insitution, res.1))
}

#[derive(Properties, PartialEq)]
struct AccountProp {
    // TODO add access_token
    account: (Institution, Vec<Account>),
    auth_key: String,
    user_id: String,
}

async fn get_access_token(auth_key: &str, account_ids: Vec<String>) -> Result<String, String> {
    let client = get_supbase_client();
    let res = client
        .from("access_tokens")
        .auth(auth_key)
        .select("access_token,plaid_accounts!inner(*)")
        .in_("plaid_accounts.account_id", account_ids)
        .single()
        .execute()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;

    #[derive(Deserialize)]
    struct Row {
        access_token: String,
    }

    let row = res.json::<Row>().await.map_err(|e| e.to_string())?;
    Ok(row.access_token)
}

#[function_component(AccountItem)]
fn account(props: &AccountProp) -> Html {
    let settings_state = use_bool_toggle(false);

    let open_dropdown = {
        let toggle = settings_state.clone();

        Callback::from(move |_| {
            toggle.toggle();
        })
    };

    let sync_account = {
        let account = props.account.clone();
        let auth_key = props.auth_key.clone();

        Callback::from(move |_| {
            let account = account.clone();
            let auth_key = auth_key.clone();

            let account_ids: Vec<String> = account.1.iter().map(|a| a.account_id.clone()).collect();
            spawn_local(async move {
                let access_token = get_access_token(&auth_key, account_ids).await.unwrap();
                let res = commands::transactions_sync(&auth_key, &access_token).await;
                log::info!("{:?}", res);
            });
        })
    };

    let remove_account = {
        let account = props.account.clone();
        let user_id = props.user_id.clone();
        let auth_key = props.auth_key.clone();

        Callback::from(move |_| {
            let user_id = user_id.clone();
            let auth_key = auth_key.clone();
            let account_ids: Vec<String> = account
                .1
                .clone()
                .iter()
                .map(|a| a.account_id.clone())
                .collect();

            spawn_local(async move {
                let res = get_access_token(&auth_key, account_ids).await;

                match res {
                    Ok(access_token) => {
                        let res =
                            commands::invokeRemoveAccount(&user_id, &auth_key, &access_token).await;
                        log::debug!("{:?}", res);
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                    }
                }
            });
        })
    };

    html! {
        <div class="m-3 card">
            <div class="card-header">
                <h1 class="card-header-title">{props.account.0.name.clone()}</h1>
                <div class="dropdown is-active">
                    <div class="dropdown-trigger">
                        <button onclick={open_dropdown} class="card-header-icon" aria-label="more options">
                            <span class="icon">
                                <i class="fas fa-cog" aria-hidden="true"></i>
                            </span>
                        </button>
                    </div>

                    if  *settings_state {
                        <div class="dropdown-menu">
                            <div class="dropdown-content">
                                <a onclick={remove_account} class="dropdown-item">{"Remove Account"}</a>
                            </div>

                            <div class="dropdown-content">
                                <a onclick={sync_account} class="dropdown-item">{"Sync Account"}</a>
                            </div>
                        </div>
                    }
                </div>
            </div>

            <div class="card-content">
                {
                    props.account.1.clone().into_iter().map(|account| {
                        html!{
                            <div>
                                {account.official_name}
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
