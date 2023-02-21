use std::collections::HashMap;

use recurr_core::{Account, Institution, SchemaAccessToken};
use serde::Deserialize;
use yew::{
    function_component, html, platform::spawn_local, Callback, Component, Html, Properties,
    UseReducerHandle,
};
use yew_hooks::use_bool_toggle;

use crate::{commands, context::Session, plaid::Link, supabase::get_supbase_client};

pub struct AccountsView {
    accounts: HashMap<Institution, Vec<Account>>,
    error: String,
}

pub enum Msg {
    GetAccounts,
    GotAccounts(HashMap<Institution, Vec<Account>>),

    Error(String),
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

        html! {
            <div>
                <div class="is-flex is-flex-direction-row is-justify-content-space-around is-align-items-center">
                    <h1 class="is-size-3">{"All Accounts"}</h1>
                    <Link />
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
                let user_id = supabase_session.user.id;

                ctx.link().send_future(async move {
                    let client = get_supbase_client();
                    let res = client
                        .from("access_tokens")
                        .auth(&auth_key)
                        .select("*,plaid_accounts(*)")
                        .eq("user_id", &user_id)
                        .execute()
                        .await;

                    if let Err(e) = res {
                        return Msg::Error(e.to_string());
                    }

                    let rows: Vec<SchemaAccessToken> =
                        res.unwrap().json().await.expect("Failed to deserialize");

                    let mut grouped_accounts: HashMap<String, Vec<String>> = HashMap::new();

                    rows.into_iter().for_each(|a| {
                        grouped_accounts.insert(
                            a.access_token,
                            a.plaid_accounts
                                .unwrap_or_default()
                                .into_iter()
                                .map(|a| a.account_id)
                                .collect(),
                        );
                    });

                    let mut all_accounts = HashMap::new();
                    for (token, ids) in grouped_accounts {
                        let accounts =
                            commands::get_accounts(&auth_key, &token, ids.to_owned()).await;

                        if let Err(e) = &accounts {
                            if let recurr_core::Error::Plaid(e) = e {
                                if &e.error_code == "ITEM_LOGIN_REQUIRED" {
                                    log::info!("Needs login");

                                    return Msg::Error("Needs an login".to_string());
                                }
                            } else {
                                return Msg::Error(e.to_string());
                            }
                        }
                        let res = accounts.unwrap();

                        let accounts = res.1;

                        let institution_id = res.0.institution_id.unwrap();
                        let insitution = commands::get_institution(&auth_key, Some(institution_id))
                            .await
                            .unwrap();

                        all_accounts.insert(insitution, accounts);
                    }

                    Msg::GotAccounts(all_accounts)
                })
            }
            Msg::GotAccounts(a) => self.accounts = a,
            Msg::Error(e) => self.error = e,
        }

        true
    }
}

#[derive(Properties, PartialEq)]
struct AccountProp {
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
