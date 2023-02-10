use std::collections::HashMap;

use recurr_core::{SchemaAccessToken, Transaction};
use yew::{html, Component, Context, Html, Properties, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

mod add_modal;

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

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let session = ctx.props().session.clone();

        html! {
            <>
            <div>
                <div class="is-flex is-justify-content-space-around is-align-items-center">
                    <h1 class="is-size-3">{"Budgets"}</h1>
                    </div>

                <add_modal::Modal {session}/>

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