mod login;

use login::LoginComponent;
use supabase_js_rs::SupabaseClient;
use yew::{html, Component, Html, Properties};

use crate::context::SessionContext;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub client: SupabaseClient,
    pub auth_cb: yew::Callback<AuthMessage>,
}

#[derive(Properties, PartialEq)]
pub struct AuthProps {
    pub context: SessionContext,
}

pub enum AuthMessage {
    MagicLinkSent,
}

enum FormType {
    MagicLink,
    Login,
}

pub struct AuthComponent {
    form_type: FormType,
    auth_cb: yew::Callback<AuthMessage>,
}

impl Component for AuthComponent {
    type Message = AuthMessage;
    type Properties = AuthProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let auth_cb = ctx.link().callback(|msg: AuthMessage| msg);

        Self {
            form_type: FormType::Login,
            auth_cb,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let auth_cb = self.auth_cb.clone();
        let client = ctx.props().context.supabase_client.clone();

        html! {
            <div class="hero is-fullheight is-flex is-justify-content-center is-align-content-center is-align-items-center">
                <div class="has-shadow has-radius p-3">
                    {
                        match self.form_type {
                            FormType::Login => html!{<LoginComponent {client} {auth_cb} />},
                            FormType::MagicLink => html!{"Magic Link Sent"}
                        }
                    }
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AuthMessage::MagicLinkSent => self.form_type = FormType::MagicLink,
        }

        true
    }
}
