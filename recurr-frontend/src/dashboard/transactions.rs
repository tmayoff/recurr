use supabase_js_rs::SupabaseClient;
use yew::{html, Component, Context, Properties};

use crate::{context::Session, supabase::get_supbase_client};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: Session,
}

pub enum Msg {
    GotTransactions,
    GetTransactions,
    Error(String),
}

pub struct TransactionsView {
    error: Option<String>,
}

impl TransactionsView {
    fn get_transaction(&self, ctx: &Context<Self>) {
        let session = ctx
            .props()
            .session
            .clone()
            .supabase_session
            .expect("Needs session");
        let auth_key = session.auth_key;
        let user_id = session.user.id;

        let db_client = get_supbase_client();

        ctx.link().send_future(async move {
            let res = db_client
                .from("access_tokens")
                .auth(auth_key)
                .select("*")
                .eq("user_id", user_id)
                .execute()
                .await;

            if let Err(e) = res {
                return Msg::Error(e.to_string());
            }

            Msg::GotTransactions
        });
    }
}

impl Component for TransactionsView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetTransactions);

        Self { error: None }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                {"Transaction"}

                {
                    if let Some(e) = &self.error {
                        html!{{e}}
                    } else {
                        html!{}
                    }
                }
            </>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.error = None;

        match msg {
            Msg::GetTransactions => self.get_transaction(ctx),
            Msg::GotTransactions => log::info!("Got transactions"),
            Msg::Error(e) => {
                log::info!("Got error {}", &e);
                self.error = Some(e);
            }
        }

        true
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &yew::Context<Self>) {}
}
