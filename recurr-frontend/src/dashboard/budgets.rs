use std::collections::HashMap;

use recurr_core::{SchemaAccessToken, Transaction};
use yew::{html, Component, Context, Html, Properties, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

pub enum Msg {
    GotTransactions(HashMap<String, f64>),
    GetTransactions,
    Error(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub struct BudgetsView {
    transactions: HashMap<String, f64>,
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
            for row in res {
                if let Some(accounts) = row.plaid_accounts {
                    let a: Vec<String> = accounts.into_iter().map(|a| a.account_id).collect();
                    let res = commands::get_transactions(&auth_key, &row.access_token, a).await;

                    if let Err(e) = res {
                        log::error!("{}", &e);
                        return Msg::Error(e);
                    }
                    let res = res.unwrap();

                    // FIXME: This seems to awkward
                    let mut grouped: HashMap<String, f64> = HashMap::new();
                    for t in res.1 {
                        if let Some(id) = &t.category_id {
                            if grouped.contains_key(id) {
                                let v = grouped.get_mut(id);
                                if let Some(v) = v {
                                    *v += t.amount;
                                }
                            } else {
                                grouped.insert(id.to_string(), t.amount);
                            }
                        }
                    }

                    return Msg::GotTransactions(grouped);
                }
            }

            Msg::Error("Failed to get transactions".to_string())
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
            transactions: HashMap::new(),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <>
            <div class="modal">
                <div class="modal-background"></div>
                <div class="modal-content">
                    {"Add budget here"}
                </div>
            </div>

            <div>
                <div class="is-flex is-justify-content-space-around is-align-items-center">
                    <h1 class="is-size-3">{"Budgets"}</h1>
                    </div>

                <button class="button is-success">{"Add a budget"}</button>

                <div>
                    <h1 class="is-size-5">{"Income"}</h1>

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
            Msg::GotTransactions(t) => self.transactions = t,
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::Error(err) => self.error = Some(err),
        }

        true
    }
}
