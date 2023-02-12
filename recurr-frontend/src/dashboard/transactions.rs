use recurr_core::{SchemaAccessToken, TransactionOption, Transactions};
use yew::{html, Component, Context, Html, Properties, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub enum Msg {
    GotTransactions(Transactions),
    GetTransactions,
    Error(String),
}

pub struct TransactionsView {
    transactions: Transactions,
    error: Option<String>,

    page: u64,
    visible_page_buttons: u64,
    total_transactions: u64,
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

                    let options = TransactionOption {
                        account_ids: a,
                        count: Some(25),
                        offset: None,
                    };
                    let res = commands::get_transactions(
                        &auth_key,
                        &row.access_token,
                        None,
                        None,
                        options,
                    )
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
            transactions: Transactions::default(),
            page: 1,
            visible_page_buttons: 5,
            total_transactions: 0,
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                <h1 class="is-size-3"> {"Transaction"} </h1>

                if let Some(e) = &self.error {
                    {e}
                }

                <div>

                    <table class="table is-hoverable is-full-width mb-0">
                        <thead>
                            <th>{"Data"}</th>
                            <th>{"Name"}</th>
                            <th>{"Category"}</th>
                            <th>{"Amount"}</th>
                        </thead>
                        <tbody>
                        {
                            self.transactions.transactions.clone().into_iter().map(|t| {
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

                    <div class="is-flex is-justify-content-left is-align-content-center is-align-items-center">
                        <p class="is-6 mr-3">{"Showing 25 transactions"}</p>
                        <nav class="pagination is-small is-centered" role="navigation" aria-label="pagination">
                            <a class="pagination-previous">{"Prev"}</a>
                            <a class="pagination-next">{"Next"}</a>
                            <ul class="pagination-list">
                            //   <li>
                            //       <a class="pagination-link" aria-label="Goto page 1">{"1"}</a>
                            //   </li>
                            //   <li>
                            //       <span class="pagination-ellipsis">{"..."}</span>
                            //   </li>
                            //   <li>
                            //       <a class="pagination-link" aria-label="Goto page 45">{"45"}</a>
                            //   </li>
                            //   <li>
                            //       <a class="pagination-link is-current" aria-label="Page 46" aria-current="page">{"46"}</a>
                            //   </li>
                            //   <li>
                            //       <a class="pagination-link" aria-label="Goto page 47">{"47"}</a>
                            //   </li>
                            //   <li>
                            //       <span class="pagination-ellipsis">{"..."}</span>
                            //   </li>
                            //   <li>
                            //       <a class="pagination-link" aria-label="Goto page 86">{"86"}</a>
                            //   </li>
                            </ul>
                        </nav>
                    </div>
                </div>
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
