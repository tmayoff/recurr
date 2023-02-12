use recurr_core::{Account, SchemaAccessToken, Transaction, TransactionOption};
use yew::{html, Component, Context, Html, Properties, UseReducerHandle};

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

                    let a = TransactionOption {
                        account_ids: a,
                        count: Some(25),
                        offset: None,
                    };
                    let res =
                        commands::get_transactions(&auth_key, &row.access_token, None, None, a)
                            .await;

                    match res {
                        Ok(t) => return Msg::GotTransactions(t),
                        Err(e) => {
                            log::error!("{}", &e);
                            return Msg::Error(e);
                        }
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

                <table class="table is-hoverable is-full-width">
                    <thead>
                        <th>{"Data"}</th>
                        <th>{"Name"}</th>
                        <th>{"Category"}</th>
                        <th>{"Amount"}</th>
                    </thead>
                    <tbody>
                    {
                        self.transactions.1.clone().into_iter().map(|t| {
                            html!{
                                <tr>
                                    <td> {t.date}</td>
                                    <td> {t.name}</td>
                                    <td> {t.category.clone().last()}</td>
                                    {
                                        if t.amount < 0.0 {
                                            html!{<td class="has-text-success">{format!("${:.2}", t.amount)}</td>}
                                        } else {
                                            html!{<td> {format!("${:.2}", t.amount)}</td>}
                                        }
                                    }
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                </table>
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.error = None;

        match msg {
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::GotTransactions(t) => self.transactions = t,
            Msg::Error(e) => {
                log::error!("Got error: {}", &e);
                self.error = Some(e);
            }
        }

        true
    }
}
