use recurr_core::{SchemaAccessToken, TransactionOption, Transactions};
use web_sys::{HtmlElement, MouseEvent};
use yew::{html, Component, Context, Html, Properties, TargetCast, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub enum Msg {
    GotTransactions(Transactions),
    GetTransactions,

    NextPage,
    GotoPage(u64),
    PrevPage,

    Error(String),
}

pub struct TransactionsView {
    transactions: Transactions,
    error: Option<String>,

    transactions_per_page: u64,
    page: u64,
    total_pages: u64,
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

        let page = self.page as u32;
        let per_page = self.transactions_per_page as i32;
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
                        count: Some(per_page),
                        offset: Some(per_page as u32 * page),
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
            transactions_per_page: 25,
            page: 1,
            total_pages: 1,
            visible_page_buttons: 5,
            total_transactions: 0,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let next_page = ctx.link().callback(|_| Msg::NextPage);
        let prev_page = ctx.link().callback(|_| Msg::PrevPage);
        let goto_page = ctx.link().callback(|e: MouseEvent| {
            if let Some(t) = e.target_dyn_into::<HtmlElement>() {
                let page = t.get_attribute("data-page");
                match page {
                    Some(p) => Msg::GotoPage(p.parse().expect("Page not a number")),
                    None => Msg::Error("Failed to find page".to_string()),
                }
            } else {
                Msg::Error("Couldn't find page".to_string())
            }
        });

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
                        <h1 class="is-size-7 mr-2">{"Showing 25 transactions"}</h1>
                        <h1 class="is-size-7 mr-3"> {format!("{}-{} of {}", ((self.page - 1) * self.transactions_per_page + 1), (self.transactions_per_page * self.page).clamp(0, self.total_transactions), self.total_transactions)} </h1>
                        <nav class="pagination is-small is-centered" role="navigation" aria-label="pagination">
                            if self.page == 1 {
                                <a class="pagination-previous is-disabled">{"Prev"}</a>
                            } else {
                                <a class="pagination-previous" onclick={prev_page}>{"Prev"}</a>
                            }
                            if self.page == self.total_pages {
                                <a class="pagination-next is-disabled">{"Next"}</a>
                            } else {
                                <a class="pagination-next" onclick={next_page}>{"Next"}</a>
                            }
                            <ul class="pagination-list">
                            {
                                (1..self.total_pages+1).map(|p| {
                                    html!{
                                        <li>
                                            if self.page == p {
                                                <a class="pagination-link is-current" aria-label={format!("Goto page {p}")}>{p}</a>
                                            } else {
                                                <a onclick={goto_page.clone()} data-page={p.to_string()} class="pagination-link" aria-label={format!("Goto page {p}")}>{p}</a>
                                            }
                                        </li>
                                    }
                                }).collect::<Html>()
                            }
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
            Msg::GotTransactions(t) => {
                self.total_transactions = t.total_transactions;
                self.total_pages = self.total_transactions / self.transactions_per_page;
                self.transactions = t;
            }
            Msg::NextPage => {
                self.page = (self.page + 1).clamp(0, self.total_pages);
                ctx.link().send_message(Msg::GetTransactions);
            }
            Msg::PrevPage => {
                self.page = (self.page - 1).clamp(0, self.total_pages);
                ctx.link().send_message(Msg::GetTransactions);
            }
            Msg::GotoPage(p) => {
                self.page = p;
                ctx.link().send_message(Msg::GetTransactions);
            }
            Msg::Error(e) => {
                log::error!("Got error: {}", &e);
                self.error = Some(e);
            }
        }

        true
    }
}
