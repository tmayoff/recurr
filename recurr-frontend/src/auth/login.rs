use serde::Deserialize;
use supabase_js_rs::Credentials;
use web_sys::{HtmlInputElement, MouseEvent, SubmitEvent};
use yew::{html, Callback, Component, Context, Html, NodeRef};

use super::FormProps;

pub enum LoginMsg {
    LoggedIn,
    Login,
    Error(Option<String>),
}

pub struct LoginComponent {
    email: NodeRef,
    password: NodeRef,

    error: Option<String>,
}

impl Component for LoginComponent {
    type Message = LoginMsg;
    type Properties = FormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: NodeRef::default(),
            password: NodeRef::default(),
            error: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let toggle = ctx.props().toggle.clone();
        let toggle_form = { Callback::from(move |_: MouseEvent| toggle.toggle()) };

        let login = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            LoginMsg::Login
        });

        html! {
        <>
            <h1 class="is-size-3">{"Sign In"}</h1>
            <form onsubmit={login}>
                <div class="field">
                    <label class="label">{"Email"}</label>
                    <div class="control">
                        <input ref={self.email.clone()} class="input" type="email" placeholder="username"/>
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Password"}</label>
                    <div class="control">
                        <input ref={self.password.clone()} class="input" type="password" placeholder="password"/>
                    </div>
                </div>
                {
                    if let Some(e) = &self.error {
                        html!{
                            <div class="field">
                                <p class="help is-danger">{e}</p>
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }
                <div class="field">
                    <div class="control">
                        <button class="button is-link">{"Login"}</button>
                    </div>
                </div>
                <div class="field">
                    <div class="control">
                        <a onclick={toggle_form}>{"Don't have an account?"}</a>
                    </div>
                </div>
            </form>
        </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::Login => {
                let client = ctx.props().client.clone();
                let email_ref = self.email.clone();
                let pass_ref = self.password.clone();
                ctx.link().send_future(async move {
                    let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
                    let password = pass_ref.cast::<HtmlInputElement>().unwrap().value();

                    let res = client
                        .auth()
                        .sign_in_with_password(Credentials { email, password })
                        .await;

                    match res {
                        Ok(res) => {
                            #[derive(Deserialize)]
                            struct AuthError {
                                message: String,
                            }

                            #[derive(Deserialize)]
                            struct AuthResponse {
                                error: Option<AuthError>,
                            }

                            let auth_res: AuthResponse =
                                serde_wasm_bindgen::from_value(res).expect("Failed to deserialize");

                            if let Some(e) = auth_res.error {
                                return LoginMsg::Error(Some(e.message));
                            }

                            LoginMsg::LoggedIn
                        }
                        Err(e) => {
                            LoginMsg::Error(Some(e.as_string().expect("Failed to get string")))
                        }
                    }
                });
            }
            LoginMsg::Error(e) => self.error = e,
            LoginMsg::LoggedIn => log::info!("Logged in"),
        };
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
