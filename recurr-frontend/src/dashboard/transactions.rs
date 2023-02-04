use recurr_core::{Account, SchemaAccessToken, Transaction};
use yew::{html, Component, Context, Properties, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub enum Msg {
    GotTransactions((Vec<Account>, Vec<Transaction>)),
    GetTransactions,
    Error(String),
}

pub struct TransactionsView {
    transactions: (Vec<Account>, Vec<Transaction>),
    error: Option<String>,
}

impl TransactionsView {
    fn get_transaction(&self, ctx: &Context<Self>) {
        let session = ctx
            .props()
            .session
            .clone()
            .supabase_session
            .clone()
            .expect("Needs session");
        let auth_key = session.auth_key;
        let user_id = session.user.id;

        let db_client = get_supbase_client();

        ctx.link().send_future(async move {
            let res = db_client
                .from("access_tokens")
                .auth(&auth_key)
                .select("*,plaid_accounts(*)")
                .eq("user_id", user_id)
                .execute()
                .await;

            if let Err(e) = res {
                return Msg::Error(e.to_string());
            }

            let res: Vec<SchemaAccessToken> =
                res.unwrap().json().await.expect("Failed to get json");

            // Get Transactions
            for row in res {
                if let Some(accounts) = row.plaid_accounts {
                    let a: Vec<String> = accounts.into_iter().map(|a| a.account_id).collect();
                    let res = commands::get_transactions(&auth_key, &row.access_token, a).await;
                    match res {
                        Ok(t) => return Msg::GotTransactions(t),
                        Err(e) => return Msg::Error(e),
                    }
                }
            }

            Msg::Error("Failed to get transactions".to_string())
        });
    }
}

impl Component for TransactionsView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetTransactions);

        Self {
            error: None,
            transactions: (Vec::new(), Vec::new()),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                <h1 class="is-size-3"> {"Transaction"} </h1>

                {
                    if let Some(e) = &self.error {
                        html!{{e}}
                    } else {
                        html!{}
                    }
                }

                {
                    html!{{format!("{:?}", self.transactions)}}
                }
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.error = None;

        match msg {
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::GotTransactions(t) => self.transactions = t,
            Msg::Error(e) => {
                log::info!("Got error: {}", &e);
                self.error = Some(e);
            }
        }

        true
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {}
}
