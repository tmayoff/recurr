use recurr_core::{Account, Institution};
use serde::Deserialize;
use yew::{
    function_component, html, platform::spawn_local, Callback, Component, Html, Properties,
    UseReducerHandle,
};
use yew_hooks::use_bool_toggle;

use crate::{
    commands::get_all_accounts, context::Session, plaid::Link, supabase::get_supbase_client,
};

pub struct AccountsView {
    accounts: Vec<(Institution, Vec<Account>)>,
    error: String,
}

pub enum Msg {
    GetAccounts,
    GotAccounts(Vec<(Institution, Vec<Account>)>),

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
            accounts: Vec::new(),
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
                            html!{<AccountItem account={a} auth_key={auth_key.clone()} />}
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

                ctx.link().send_future(async move {
                    let res = get_all_accounts(&auth_key, &user_id).await;

                    match res {
                        Ok(accounts) => Msg::GotAccounts(accounts),
                        Err(e) => Msg::Error(e),
                    }
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
}

async fn get_access_token(auth_key: String, account_ids: Vec<String>) -> Result<String, String> {
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
        let auth_key = props.auth_key.clone();

        Callback::from(move |_| {
            let auth_key = auth_key.clone();
            let account_ids: Vec<String> = account
                .1
                .clone()
                .iter()
                .map(|a| a.account_id.clone())
                .collect();

            log::info!("Delete account");

            spawn_local(async move {
                let res = get_access_token(auth_key, account_ids).await;
                log::debug!("{:?}", res);

                // TODO Remove from plaid
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
