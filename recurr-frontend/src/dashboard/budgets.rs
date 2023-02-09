use std::collections::HashMap;

use recurr_core::{SchemaAccessToken, Transaction};
use wasm_bindgen::prelude::Closure;
use web_sys::{HtmlElement, MouseEvent};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

pub enum Msg {
    GotTransactions((f64, Vec<Transaction>, Vec<Transaction>)),
    GetTransactions,
    Error(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub struct BudgetsView {
    income: f64,
    transactions: HashMap<String, f64>,
    error: Option<String>,
    modal_ref: NodeRef,
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

            let income: f64 = transactions
                .into_iter()
                .filter(|t| t.amount > 0.0)
                .map(|t| t.amount)
                .sum();

            let mut grouped: HashMap<String, f64> = HashMap::new();
            // for t in transactions {
            //     if let Some(id) = &t.category_id {
            //         if grouped.contains_key(id) {
            //             let v = grouped.get_mut(id);
            //             if let Some(v) = v {
            //                 *v += t.amount;
            //             }
            //         } else {
            //             grouped.insert(id.to_string(), t.amount);
            //         }
            //     }
            // }

            Msg::GotTransactions((income, Vec::new(), Vec::new()))
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
            income: 0.0,
            transactions: HashMap::new(),
            modal_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let div = self.modal_ref.clone();
        let open_add_modal = move |_| {
            let div = div
                .clone()
                .cast::<HtmlElement>()
                .expect("Failed to html element");

            div.set_class_name("modal is-active");
        };

        let div = self.modal_ref.clone();
        let close_add_modal = Callback::from(move |_| {
            let div = div
                .clone()
                .cast::<HtmlElement>()
                .expect("Failed to html element");

            div.set_class_name("modal");
        });

        html! {
            <>
            <div class="modal" ref={self.modal_ref.clone()}>
                <div class="modal-background" onclick={close_add_modal.clone()}></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Add budget"}</p>
                        <button class="delete" aria-label="close"></button>
                    </header>
                    <section class="modal-card-body">
                        <form>
                            <div class="select is-info">
                                <select placeholder="Choose a category">
                                </select>
                            </div>

                            <div class="field">
                                <label class="label">{"How much"}</label>
                                <div class="control">
                                    <input class="input is-success" type="number" value="0" />
                                </div>
                            </div>
                        </form>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button" onclick={close_add_modal.clone()}>{"Cancel"}</button>
                        <button class="button is-success">{"Save"}</button>
                    </footer>
                </div>
            </div>

            <div>
                <div class="is-flex is-justify-content-space-around is-align-items-center">
                    <h1 class="is-size-3">{"Budgets"}</h1>
                    </div>

                <button class="button is-success" onclick={open_add_modal}>{"Add a budget"}</button>

                <div>
                    <h1 class="is-size-5">{"Income"}</h1>
                    <p>{format!("${:0.2}", self.income)}</p>

                    <h1 class="is-size-5">{"Spending"}</h1>

                    {
                        self.transactions.clone().into_iter().map(|(cat, amount)| {
                            html!{
                                <div>
                                    <h1>{cat}</h1>
                                    <div class="p-3">
                                        <p>{format!("${amount:0.2}")}</p>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }

                </div>
            </div>
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotTransactions(t) => self.income = t.0,
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::Error(err) => self.error = Some(err),
        }

        true
    }
}
