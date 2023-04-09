use std::collections::HashMap;

use chrono::Local;
use now::DateTimeNow;
use recurr_core::{
    get_supbase_client, SchemaAccessToken, SchemaBudget, Transaction, TransactionOption,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::{
    html, Callback, Component, Context, ContextHandle, Html, Properties, TargetCast,
    UseReducerHandle,
};

use crate::{
    commands,
    context::{Session, SessionContext},
};

use super::{transactions::Filter, DashboardTab};

mod edit_modal;

#[derive(Default)]
pub struct Transactions {
    other_income: HashMap<String, f64>,
    budgeted_spending: Vec<(SchemaBudget, f64)>,
    other_spending: HashMap<String, f64>,
}

pub enum Msg {
    UpdatedContext(SessionContext),

    ShowModal(Option<SchemaBudget>),
    HideModal,

    GotTransactions(Transactions),
    GetTransactions,

    Update,

    Error(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub context: UseReducerHandle<Session>,
    pub switch_tab: Callback<DashboardTab>,
}

pub struct BudgetsView {
    context: SessionContext,
    _context_listener: ContextHandle<SessionContext>,

    transactions: Transactions,
    error: Option<String>,

    budget_details: Option<SchemaBudget>,

    modal_show: bool,
}

impl BudgetsView {
    fn get_transaction(&self, ctx: &Context<Self>) {
        // TODO Clean this up

        let session = ctx
            .props()
            .context
            .clone()
            .supabase_session
            .clone()
            .expect("Needs session");
        let auth_key = session.auth_key;
        let user_id = session.user.id;

        ctx.link().send_future(async move {
            let transactions = get_transactions(&auth_key).await;
            if let Err(e) = transactions {
                return Msg::Error(e.to_string());
            }
            let transactions = transactions.unwrap();

            let budgets = get_budgets(&auth_key, &user_id).await;
            if let Err(e) = budgets {
                return Msg::Error(e.to_string());
            }
            let budgets = budgets.unwrap();

            let income: Vec<Transaction> = Vec::new();
            let mut spending: Vec<Transaction> = transactions;

            let mut budgeted_spending = Vec::new();
            for b in budgets {
                let mut amount = 0.0;

                let budgeted: Vec<Transaction> = spending
                    .drain_filter(|t| t.category.as_ref().unwrap().contains(&b.category_id))
                    .collect();
                budgeted.into_iter().for_each(|t| {
                    amount += t.amount;
                });

                budgeted_spending.push((b.clone(), amount));
            }
            budgeted_spending.sort_by(|a, b| a.0.category_id.cmp(&b.0.category_id));

            let mut other_spending: HashMap<String, f64> = HashMap::new();
            for t in spending {
                let general_category = t.category.as_ref().unwrap().first();
                if let Some(category) = general_category {
                    if other_spending.contains_key(category) {
                        let v = other_spending.get_mut(category);
                        if let Some(v) = v {
                            *v += t.amount;
                        }
                    } else {
                        other_spending.insert(category.to_string(), t.amount);
                    }
                }
            }

            Msg::GotTransactions(Transactions {
                other_income: HashMap::new(),
                budgeted_spending,
                other_spending: HashMap::new(),
            })
        });
    }
}

impl Component for BudgetsView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let (context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::UpdatedContext))
            .expect("No context provided");

        ctx.link().send_message(Msg::GetTransactions);

        Self {
            transactions: Transactions::default(),
            error: None,
            budget_details: None,
            modal_show: false,
            context,
            _context_listener: context_listener,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let session = ctx.props().context.clone();

        let modal_cb = ctx.link().callback(|e: edit_modal::ModalMsg| match e {
            edit_modal::ModalMsg::Close => Msg::HideModal,
            edit_modal::ModalMsg::Save => Msg::Update,
        });

        let edit_budget = ctx.link().callback(move |e: MouseEvent| {
            let target = e.target();

            let t = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());

            if let Some(t) = t {
                let category = t.get_attribute("data-category");
                let max = t.get_attribute("data-amount");

                if let (Some(category_id), Some(max)) = (category, max) {
                    let max: f64 = max.parse().expect("Failed to parse budget max");

                    let b = SchemaBudget {
                        user_id: "".to_string(),
                        category_id,
                        max,
                    };

                    Msg::ShowModal(Some(b))
                } else {
                    Msg::ShowModal(None)
                }
            } else {
                Msg::Error("Failed to edit modal".to_string())
            }
        });

        let goto_transactions = {
            let switch_tabs = ctx.props().switch_tab.clone();
            Callback::from(move |e: MouseEvent| {
                let cat_element = e.target_dyn_into::<HtmlElement>().unwrap();
                let cat = cat_element.get_attribute("data-category").unwrap();

                let start_date = Local::now().beginning_of_month();
                let end_date = Local::now().end_of_month();
                switch_tabs.emit(DashboardTab::Transaction(Filter {
                    start_date: Some(start_date.naive_local().format("%Y-%m-%d").to_string()),
                    end_date: Some(end_date.naive_local().format("%Y-%m-%d").to_string()),
                    category: Some(cat),
                }));
            })
        };

        html! {
            <>
            <div class="column">
                <div class="is-flex is-justify-content-space-around is-align-items-center">
                    <h1 class="title">{"Budgets"}</h1>
                </div>

                <button class="button is-success" onclick={edit_budget.clone()}>{"Add Budget"}</button>
                <edit_modal::Modal on_change={modal_cb} {session} show={self.modal_show} detail={self.budget_details.clone()}/>

                <div class="columns m-1">
                    <div class="column is-half is-flex is-flex-direction-column">

                        <div>
                            <h1 class="is-size-5">{"Income"}</h1>
                            {
                                if !self.transactions.other_income.is_empty() {
                                    html!{
                                        <div>
                                            <table class="table">
                                                <thead>
                                                    <th>{"Other income"}</th>
                                                </thead>
                                                <tbody>
                                                {
                                                    self.transactions.other_income.clone().into_iter().map(|(c, a)| {
                                                        html!{
                                                            <tr>
                                                                <td>{c}</td>
                                                                <td>{format!("${:0.2}", a.abs())}</td>
                                                                // <td><button class="button">{"+"}</button></td>
                                                            </tr>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                                </tbody>
                                            </table>
                                        </div>
                                    }
                                } else {
                                    html!{}
                                }
                            }
                        </div>

                        <div class="separator m-1"></div>

                        <div>
                            <h1 class="is-size-5">{"Spending"}</h1>
                        {
                            if !self.transactions.budgeted_spending.is_empty() {
                                html!{
                                    {
                                        self.transactions.budgeted_spending.clone().into_iter().map(|(c, a)| {
                                            html!{
                                                <div>
                                                    <div class="is-flex is-justify-content-space-between">
                                                        <td><a class="has-hover-underline" data-category={c.category_id.clone()} onclick={goto_transactions.clone()}> {c.category_id.clone()} </a></td>
                                                        <div>{format!("${:0.2} left", c.max - a)}</div>
                                                    </div>
                                                    <progress class="progress m-0 is-success" value={format!("{:0.2}", a/c.max)} max="1">{format!("{:0.2}", a/c.max)}</progress>
                                                    <div class="is-flex is-justify-content-flex-end">
                                                        <a onclick={edit_budget.clone()} data-category={c.category_id} data-amount={format!("{:0.2}", c.max)}>{"Edit"}</a>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                }
                            } else {
                                html!{}
                            }
                        }
                        </div>

                        {
                            if !self.transactions.other_spending.is_empty() {
                                html!{
                                    <div>
                                        <table class="table">
                                            <thead>
                                                <th>{"Other spending"}</th>
                                            </thead>
                                            <tbody>
                                            {
                                                self.transactions.other_spending.clone().into_iter().map(|(c, a)| {
                                                    html!{
                                                        <tr>
                                                            <td>{c}</td>
                                                            <td>{format!("${a:0.2}")}</td>
                                                            // <td><button class="button">{"+"}</button></td>
                                                        </tr>
                                                    }
                                                }).collect::<Html>()
                                            }
                                            </tbody>
                                        </table>
                                    </div>
                                }
                            } else {
                                html!{}
                            }
                        }
                    </div>
                </div>
            </div>
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotTransactions(t) => self.transactions = t,
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::Error(err) => self.error = Some(err),
            Msg::ShowModal(b) => {
                self.budget_details = b;
                self.modal_show = true;
            }
            Msg::HideModal => self.modal_show = false,
            Msg::Update => self.get_transaction(ctx),
            Msg::UpdatedContext(context) => self.context = context,
        }

        true
    }
}

async fn get_transactions(auth_key: &str) -> Result<Vec<Transaction>, recurr_core::Error> {
    let db_client = get_supbase_client();

    let res = db_client
        .from("transactions")
        .auth(&auth_key)
        .select("*")
        .order("date.desc")
        .execute()
        .await?
        .error_for_status()?;

    Ok(res.json().await?)
}

async fn get_budgets(
    auth_key: &str,
    user_id: &str,
) -> Result<Vec<SchemaBudget>, recurr_core::Error> {
    let db_client = get_supbase_client();

    let res = db_client
        .from("budgets")
        .auth(&auth_key)
        .select("*")
        .eq("user_id", &user_id)
        .execute()
        .await?
        .error_for_status()?;

    Ok(res.json().await?)
}
