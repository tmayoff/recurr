use recurr_core::{Account, Institution};
use yew::{function_component, html, Component, Html, Properties, UseReducerHandle};

use crate::{commands::get_all_accounts, context::Session, plaid::Link};

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

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
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
                            html!{<AccountItem account={a} />}
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
}

#[function_component(AccountItem)]
fn account(props: &AccountProp) -> Html {
    html! {
        <div class="m-3 card">
            <div class="card-header">
                <h1 class="card-header-title">{props.account.0.name.clone()}</h1>
                <button class="card-header-icon" aria-label="more options">
                    <span class="icon">
                        <i class="fas fa-cog" aria-hidden="true"></i>
                    </span>
                </button>
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
