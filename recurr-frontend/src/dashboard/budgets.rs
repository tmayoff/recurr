use std::collections::HashMap;

use recurr_core::{Category, SchemaAccessToken, Transaction};
use web_sys::{HtmlElement, HtmlInputElement, SubmitEvent};
use yew::{html, Component, Context, Html, NodeRef, Properties, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

enum BudgetModalMsg {
    GotCategories(Vec<Category>),
    OpenModal,
    CloseModal,
    Submit,
}

struct BudgetModal {
    modal_ref: NodeRef,
    categories: Vec<Category>,

    category_ref: NodeRef,
    amount_ref: NodeRef,
}

impl BudgetModal {
    async fn get_categories() -> BudgetModalMsg {
        let categories = commands::get_categories()
            .await
            .expect("Failed to get categories");
        //TODO Would be good to either sort this or group it
        BudgetModalMsg::GotCategories(categories)
    }

    fn set_modal_class(&self, class: &str) {
        let elem = self.modal_ref.cast::<HtmlElement>();
        if let Some(elem) = elem {
            elem.set_class_name(class);
        }
    }
}

impl Component for BudgetModal {
    type Message = BudgetModalMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(Self::get_categories());

        Self {
            modal_ref: NodeRef::default(),
            categories: Vec::new(),

            category_ref: NodeRef::default(),
            amount_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_modal = ctx.link().callback(|_| BudgetModalMsg::CloseModal);
        let open_modal = ctx.link().callback(|_| BudgetModalMsg::OpenModal);
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            BudgetModalMsg::Submit
        });

        html! {
            <>
            <div class="modal" ref={self.modal_ref.clone()}>
                <div class="modal-background" onclick={close_modal.clone()}></div>
                <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{"Add budget"}</p>
                    <button class="delete" aria-label="close" onclick={close_modal.clone()}></button>
                </header>
                {
                    if !self.categories.is_empty() {
                        html!{
                            <>
                            <section class="modal-card-body">
                                <form onsubmit={on_submit}>
                                    <div class="select is-info">
                                        <select placeholder="Choose a category" ref={self.category_ref.clone()}>
                                            {
                                                self.categories.clone().iter().map(|c| {
                                                    html!{<option>{c.hierarchy.last()}</option>}
                                                }).collect::<Html>()
                                            }
                                        </select>
                                    </div>

                                    <div class="field">
                                        <label class="label">{"How much"}</label>
                                        <div class="control">
                                            <input class="input is-success" type="number" value="0" ref={self.amount_ref.clone()}/>
                                        </div>
                                    </div>
                                </form>
                            </section>
                            <footer class="modal-card-foot">
                                <button class="button" onclick={close_modal.clone()}>{"Cancel"}</button>
                                <button class="button is-success">{"Save"}</button>

                            </footer>
                            </>
                        }
                    } else {
                        html!{}
                    }
                }
                </div>
            </div>

            <button class="button is-success" onclick={open_modal}>{"Add a budget"}</button>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BudgetModalMsg::GotCategories(categories) => self.categories = categories,
            BudgetModalMsg::OpenModal => self.set_modal_class("modal is-active"),
            BudgetModalMsg::CloseModal => self.set_modal_class("modal"),
            BudgetModalMsg::Submit => {
                let _amount = self
                    .amount_ref
                    .cast::<HtmlInputElement>()
                    .expect("Amount ref not an input element");

                let _category = self
                    .category_ref
                    .cast::<HtmlInputElement>()
                    .expect("Category ref not an input element");

                // TODO submit to database
            }
        }

        true
    }
}

pub enum Msg {
    GotTransactions((HashMap<String, f64>, HashMap<String, f64>, Vec<Transaction>)),
    GetTransactions,
    Error(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub struct BudgetsView {
    other_income: HashMap<String, f64>,
    other_spending: HashMap<String, f64>,
    error: Option<String>,
}

impl BudgetsView {
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
            let mut transactions = Vec::new();
            for row in res {
                if let Some(accounts) = row.plaid_accounts {
                    let a: Vec<String> = accounts.into_iter().map(|a| a.account_id).collect();
                    let res = commands::get_transactions(&auth_key, &row.access_token, a).await;

                    if let Err(e) = res {
                        log::error!("{}", &e);
                        return Msg::Error(e);
                    }

                    let res = res.unwrap();
                    transactions.extend(res.1.clone());
                }
            }

            let income: Vec<Transaction> = transactions
                .clone()
                .into_iter()
                .filter(|t| t.amount < 0.0)
                .collect();

            let spending: Vec<Transaction> = transactions
                .into_iter()
                .filter(|t| t.amount > 0.0)
                .collect();

            let mut grouped_income: HashMap<String, f64> = HashMap::new();
            income.into_iter().for_each(|t| {
                let category = t.category.first();
                if let Some(category) = category {
                    if grouped_income.contains_key(category) {
                        let v = grouped_income.get_mut(category);
                        if let Some(v) = v {
                            *v += t.amount;
                        }
                    } else {
                        grouped_income.insert(category.to_string(), t.amount);
                    }
                }
            });

            let mut grouped_spending: HashMap<String, f64> = HashMap::new();
            for t in spending {
                let category = t.category.first();
                if let Some(category) = category {
                    if grouped_spending.contains_key(category) {
                        let v = grouped_spending.get_mut(category);
                        if let Some(v) = v {
                            *v += t.amount;
                        }
                    } else {
                        grouped_spending.insert(category.to_string(), t.amount);
                    }
                }
            }

            Msg::GotTransactions((grouped_income, grouped_spending, Vec::new()))
        });
    }
}

impl Component for BudgetsView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetTransactions);

        Self {
            error: None,
            other_income: HashMap::new(),
            other_spending: HashMap::new(),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <>
            <div>
                <div class="is-flex is-justify-content-space-around is-align-items-center">
                    <h1 class="is-size-3">{"Budgets"}</h1>
                    </div>

                <BudgetModal />

                <div>
                    <h1 class="is-size-5">{"Income"}</h1>
                    {
                        if !self.other_income.is_empty() {
                            html!{
                                <div>
                                    <table class="table">
                                        <thead>
                                            <th>{"Other income"}</th>
                                        </thead>
                                        <tbody>
                                        {
                                            self.other_income.clone().into_iter().map(|(c, a)| {
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

                    <h1 class="is-size-5">{"Spending"}</h1>

                    {
                        if !self.other_spending.is_empty() {
                            html!{
                                <div>
                                    <table class="table">
                                        <thead>
                                            <th>{"Other spending"}</th>
                                        </thead>
                                        <tbody>
                                        {
                                            self.other_spending.clone().into_iter().map(|(c, a)| {
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
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotTransactions(t) => {
                self.other_income = t.0;
                self.other_spending = t.1;
            }
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::Error(err) => self.error = Some(err),
        }

        true
    }
}
