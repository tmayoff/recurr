use recurr_core::Account;
use yew::{
    function_component, html, Callback, Component, Context, Html, Properties, UseReducerHandle,
};
use yew_hooks::use_bool_toggle;

use crate::{commands, context::Session};

#[derive(Default)]
pub struct Balances {
    cash: (Vec<Account>, f64),
    credit: (Vec<Account>, f64),
    investments: (Vec<Account>, f64),
    loans: (Vec<Account>, f64),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub context: UseReducerHandle<Session>,
}

pub enum Msg {
    GotBalances(Balances),
    GetBalances,

    Error(String),
}

#[derive(Default)]
pub struct SummaryView {
    balances: Option<Balances>,
}

impl SummaryView {
    fn get_balances(&self, ctx: &Context<Self>) {
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
            let balances = commands::get_balances(&auth_key, &user_id).await;
            match balances {
                Ok(b) => {
                    let mut balances = Balances::default();

                    for account in &b {
                        if account.account_type == "investment" {
                            balances.investments.0.push(account.clone());
                            balances.investments.1 += account.balances.current.unwrap();
                        }

                        if account.account_type == "credit" {
                            balances.credit.0.push(account.clone());
                            balances.credit.1 += account.balances.current.unwrap();
                        }

                        if account.account_type == "loan" {
                            balances.loans.0.push(account.clone());
                            balances.loans.1 += account.balances.current.unwrap();
                        }

                        if account.account_type == "depository" {
                            balances.cash.0.push(account.clone());
                            balances.cash.1 += account.balances.current.unwrap();
                        }
                    }

                    Msg::GotBalances(balances)
                }
                Err(e) => Msg::Error(e),
            }
        });
    }
}

impl Component for SummaryView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetBalances);

        Self::default()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
                {
                if let Some(balances) = &self.balances {
                    html!{
                        <div class="columns">
                            <div class="column is-narrow">
                                <Summary name={"Cash"} accounts={balances.cash.0.clone()} total={balances.cash.1}/>
                                <Summary name={"Credit Cards"} accounts={balances.credit.0.clone()} total={balances.credit.1}/>
                                <Summary name={"Investments"} accounts={balances.investments.0.clone()} total={balances.investments.1}/>
                                <Summary name={"Loans"} accounts={balances.loans.0.clone()} total={balances.loans.1}/>
                            </div>
                        </div>
                    }
                } else {
                    html!{
                        <progress class="progress is-small is-primary" max="100">{"15%"}</progress>
                    }
                }
            }
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetBalances => self.get_balances(ctx),
            Msg::GotBalances(b) => self.balances = Some(b),
            Msg::Error(e) => log::error!("{e}"),
        }

        true
    }
}

#[derive(Properties, PartialEq)]
struct SummaryDetails {
    name: String,
    accounts: Vec<Account>,
    total: f64,
}

#[function_component(Summary)]
fn summary(props: &SummaryDetails) -> Html {
    let body_open = use_bool_toggle(false);

    let onclick = {
        let toggle = body_open.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <div class="card m-3">
            <header class="card-header">
                <div class="card-header-title is-flex is-justify-content-space-between">
                    <p class="mr-4">{props.name.clone()}</p>
                    <p>{format!("${:.2}", props.total)}</p>
                </div>
                <button {onclick} class="card-header-icon" aria-label="more options">
                    <span class="icon">
                        <i class="fas fa-angle-down" aria-hidden="true"></i>
                    </span>
                </button>
            </header>
            if *body_open {
                <div class="card-content is-flex is-flex-direction-column">
                    {
                        props.accounts.clone().into_iter().map(|a| {
                            html!{
                                <div class="is-flex is-flex is-justify-content-space-between">
                                    <h1>{a.name}</h1>
                                    <h1>{format!("${:.2}", a.balances.current.unwrap_or(0.0))}</h1>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </div>
    }
}
