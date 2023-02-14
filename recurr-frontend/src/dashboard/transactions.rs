use chrono::NaiveDate;
use recurr_core::{SchemaAccessToken, TransactionOption, Transactions};
use serde::{Deserialize, Serialize};
use web_sys::{HtmlElement, HtmlInputElement, MouseEvent};
use yew::{
    function_component, html, use_node_ref, Callback, Component, Context, ContextHandle, Html,
    Properties, TargetCast, UseReducerHandle,
};
use yew_hooks::use_bool_toggle;

use crate::{
    commands,
    components::pagination::Paginate,
    context::{Session, SessionContext},
    supabase::get_supbase_client,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub context: UseReducerHandle<Session>,
    pub filter: Filter,
}

pub enum Msg {
    UpdatedContext(SessionContext),

    GotTransactions(Transactions),
    GetTransactions,
    SetFilter(Filter),

    NextPage,
    GotoPage(u64),
    PrevPage,

    Error(String),
}

pub struct TransactionsView {
    context: SessionContext,
    _context_listener: ContextHandle<SessionContext>,

    filter: Filter,
    transactions: Transactions,
    error: Option<String>,

    transactions_per_page: u64,
    page: u64,
    total_pages: u64,
    total_transactions: u64,
}

impl TransactionsView {
    fn get_transaction(&self, ctx: &Context<Self>) {
        let session = ctx
            .props()
            .context
            .clone()
            .supabase_session
            .clone()
            .expect("Needs session");
        let auth_key = session.auth_key;
        let user_id = session.user.id;

        let db_client = get_supbase_client();

        let page = self.page as u32;
        let per_page = self.transactions_per_page as i32;
        let start_date = self.filter.start_date.clone();
        let end_date = self.filter.end_date.clone();

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

            let start_date = start_date.map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap());
            let end_date = end_date.map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap());

            // Get Transactions
            for row in res {
                if let Some(accounts) = row.plaid_accounts {
                    let a: Vec<String> = accounts.into_iter().map(|a| a.account_id).collect();

                    let options = TransactionOption {
                        account_ids: a,
                        count: Some(per_page),
                        offset: Some(per_page as u32 * (page - 1)),
                    };
                    let res = commands::get_transactions(
                        &auth_key,
                        &row.access_token,
                        start_date,
                        end_date,
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
        let (context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::UpdatedContext))
            .expect("No context provided");

        ctx.link().send_message(Msg::GetTransactions);

        let filter = ctx.props().filter.clone();

        Self {
            context,
            _context_listener: context_listener,

            error: None,
            transactions: Transactions::default(),
            transactions_per_page: 25,
            page: 1,
            total_pages: 1,
            total_transactions: 0,
            filter,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let next_page = ctx.link().callback(|_| Msg::NextPage);
        let prev_page = ctx.link().callback(|_| Msg::PrevPage);
        let goto_page = ctx.link().callback(|p| Msg::GotoPage(p as u64));

        let current_page = self.page as i64;
        let total_pages = self.total_pages as i64;

        let filter_cb = ctx.link().callback(Msg::SetFilter);
        let filter = self.filter.clone();

        let cat_onclick = {
            let filter = self.filter.clone();
            ctx.link().callback(move |e: MouseEvent| {
                let mut filter = filter.clone();
                let target = e.target_dyn_into::<HtmlElement>().unwrap();
                let cat = target.get_attribute("data-category");
                filter.category = cat;

                Msg::SetFilter(filter)
            })
        };

        html! {
            <div class="column">
                <h1 class="is-size-3"> {"Transaction"} </h1>

                if let Some(e) = &self.error {
                    {e}
                }

                <div>
                    <Filters apply_filter={filter_cb} {filter}/>
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
                                let cat = t.category.last().unwrap();
                                html!{
                                    <tr>
                                        <td> {t.date}</td>
                                        <td> {t.name}</td>
                                        <td><a class="has-hover-underline" data-category={cat.clone()} onclick={cat_onclick.clone()}> {cat} </a></td>
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
                        <Paginate {next_page} {prev_page} {goto_page} {current_page} {total_pages} />
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.error = None;

        match msg {
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::GotTransactions(t) => {
                let mut transactions = t.transactions.clone();

                if let Some(cat) = &self.filter.category {
                    log::info!("Filtering");
                    transactions = transactions
                        .drain_filter(|t| t.category.last().unwrap() == cat)
                        .collect();
                }

                self.total_transactions = transactions.len() as u64;
                self.total_pages = self.total_transactions / self.transactions_per_page;

                let mut t = t;
                t.transactions = transactions;
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
            Msg::SetFilter(f) => {
                self.filter = f;
                log::info!("Set filter {:?}", self.filter);
                ctx.link().send_message(Msg::GetTransactions)
            }
            Msg::UpdatedContext(context) => self.context = context,
        }

        true
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone, Serialize)]
pub struct Filter {
    start_date: Option<String>,
    end_date: Option<String>,
    category: Option<String>,
}

#[derive(Properties, PartialEq)]
struct FilterProps {
    apply_filter: Callback<Filter>,
    filter: Filter,
}

#[function_component(Filters)]
fn filters(props: &FilterProps) -> Html {
    let open = use_bool_toggle(false);
    let start_date_ref = use_node_ref();
    let end_date_ref = use_node_ref();

    let onclick = {
        let open = open.clone();

        Callback::from(move |_| {
            open.toggle();
        })
    };

    let apply = {
        let cb = props.apply_filter.clone();
        let start_date_ref = start_date_ref.clone();
        let end_date_ref = end_date_ref.clone();

        Callback::from(move |_| {
            let start_date = start_date_ref.cast::<HtmlInputElement>().unwrap().value();
            let end_date = end_date_ref.cast::<HtmlInputElement>().unwrap().value();

            let start_date = if start_date.is_empty() {
                None
            } else {
                Some(start_date)
            };

            let end_date = if end_date.is_empty() {
                None
            } else {
                Some(end_date)
            };

            cb.emit(Filter {
                start_date,
                end_date,
                category: None,
            });
        })
    };

    let start_date = props.filter.start_date.clone().unwrap_or_default();
    let end_date = props.filter.end_date.clone().unwrap_or_default();

    html! {
        <>
        <div class="dropdown is-active">
            <div class="dropdown-trigger">
                <button {onclick} class="button" aria-haspopup="true" aria-controls="dropdown-menu">
                    <span>{"Filters"}</span>
                    <span class="icon is-small">
                        if *open {
                            <i class="fas fa-angle-up" aria-hidden="true"></i>
                        } else {
                            <i class="fas fa-angle-down" aria-hidden="true"></i>
                        }
                    </span>
                </button>
            </div>

            if *open {
                <div class="dropdown-menu" id="dropdown-menu" role="menu">
                    <div class="dropdown-content">
                        <hr class="dropdown-divider" />
                        <div class="dropdown-item">
                            <label>{"Start date"}</label>
                            <br />
                            <input ref={start_date_ref} class="input is-small" type="date" value={start_date}/>
                        </div>
                        <div class="dropdown-item">
                            <label>{"End date"}</label>
                            <br />
                            <input ref={end_date_ref} class="input is-small" type="date" value={end_date}/>
                        </div>
                        <div class="dropdown-item">
                            <button class="button is-small mr-3">{"Clear"}</button>
                            <button onclick={apply} class="button is-success is-small">{"Apply"}</button>
                        </div>
                    </div>
                </div>
            }
        </div>
        </>
    }
}
