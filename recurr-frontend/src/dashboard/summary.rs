use recurr_core::Account;
use yew::{html, Component, Context, Html, Properties, UseReducerHandle};

use crate::{commands, context::Session};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub enum Msg {
    GotBalances(Vec<Account>),
    GetBalances,

    Error(String),
}

pub struct SummaryView {
    balances: Vec<Account>,
}

impl SummaryView {
    fn get_balances(&self, ctx: &Context<Self>) {
        let auth_key = ctx
            .props()
            .session
            .clone()
            .supabase_session
            .clone()
            .unwrap()
            .auth_key;
        let user_id = ctx
            .props()
            .session
            .clone()
            .supabase_session
            .clone()
            .unwrap()
            .user
            .id;

        ctx.link().send_future(async move {
            let balances = commands::get_balances(&auth_key, &user_id).await;
            match balances {
                Ok(b) => Msg::GotBalances(b),
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

        Self {
            balances: Vec::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div>
            {
                self.balances.clone().into_iter().map(|account| {
                    html!{
                        <div class="m-3 card">
                            <div class="card-header">
                                <h1 class="card-header-title">{account.name}</h1>
                            </div>

                            <div class="card-content">
                                {"$"} {account.balances.current}
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetBalances => self.get_balances(ctx),
            Msg::GotBalances(b) => self.balances = b,
            Msg::Error(e) => log::error!("{e}"),
        }

        true
    }
}
