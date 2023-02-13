use std::collections::VecDeque;

use chrono::NaiveDate;
use recurr_core::{SchemaAccessToken, TransactionOption, Transactions};
use web_sys::{HtmlElement, HtmlInputElement, MouseEvent};
use yew::{
    function_component, html, use_node_ref, Callback, Component, Context, ContextHandle, Html,
    Properties, TargetCast,
};
use yew_hooks::use_bool_toggle;

use crate::{commands, context::SessionContext, supabase::get_supbase_client};

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
        let session = self
            .context
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
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let (context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::UpdatedContext))
            .expect("No context provided");

        ctx.link().send_message(Msg::GetTransactions);

        Self {
            context,
            _context_listener: context_listener,

            error: None,
            transactions: Transactions::default(),
            transactions_per_page: 25,
            page: 1,
            total_pages: 1,
            total_transactions: 0,
            filter: Filter::default(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let next_page = ctx.link().callback(|_| Msg::NextPage);
        let prev_page = ctx.link().callback(|_| Msg::PrevPage);

        let current_page = self.page;
        let goto_page = ctx.link().callback(move |e: MouseEvent| {
            if let Some(t) = e.target_dyn_into::<HtmlElement>() {
                let page = t.get_attribute("data-page");
                match page {
                    Some(p) => {
                        let p = p.parse().unwrap_or(current_page);
                        Msg::GotoPage(p)
                    }
                    None => Msg::Error("Failed to find page".to_string()),
                }
            } else {
                Msg::Error("Couldn't find page".to_string())
            }
        });

        let pagination = paginate(self.page as i64, self.total_pages as i64);

        let filter_cb = ctx.link().callback(Msg::SetFilter);
        let filter = self.filter.clone();

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
                                pagination.into_iter().map(|p| {
                                    html!{
                                        <li>
                                            if self.page.to_string() == p {
                                                <a class="pagination-link is-current" aria-label={format!("Goto page {p}")}>{p}</a>
                                            } else {
                                                <a onclick={goto_page.clone()} data-page={p.clone()} class="pagination-link" aria-label={format!("Goto page {p}")}>{p}</a>
                                            }
                                        </li>
                                    }
                                }).collect::<Html>()
                            }
                            </ul>
                        </nav>
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
            Msg::SetFilter(f) => {
                self.filter = f;
                ctx.link().send_message(Msg::GetTransactions)
            }
            Msg::UpdatedContext(context) => self.context = context,
        }

        true
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Filter {
    start_date: Option<String>,
    end_date: Option<String>,
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
            });
        })
    };

    let start_date = props.filter.start_date.clone().unwrap_or_default();
    let end_date = props.filter.end_date.clone().unwrap_or_default();

    html! {
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
    }
}

fn paginate(current_page: i64, page_count: i64) -> VecDeque<String> {
    const GAP: &str = "...";
    let center = vec![
        current_page - 2,
        current_page - 1,
        current_page,
        current_page + 1,
        current_page + 2,
    ];
    let mut center_deque: VecDeque<String> = center
        .iter()
        .filter(|&p| *p > 1i64 && *p < page_count)
        .map(i64::to_string)
        .collect();
    let include_three_left = current_page == 5;
    let include_three_right = current_page == page_count - 4;
    let include_left_dots = current_page > 5;
    let include_right_dots = current_page < page_count - 4;

    if include_three_left {
        center_deque.push_front("2".into());
    }
    if include_three_right {
        center_deque.push_back((page_count - 1i64).to_string());
    }
    if include_left_dots {
        center_deque.push_front(GAP.into());
    }
    if include_right_dots {
        center_deque.push_back(GAP.into());
    }
    center_deque.push_front("1".into());
    if page_count > 1i64 {
        center_deque.push_back(page_count.to_string());
    }
    center_deque
}
