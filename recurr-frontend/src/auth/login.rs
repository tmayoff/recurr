use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::{html, Component, Context, Html, NodeRef};

use super::FormProps;

pub enum LoginMsg {
    MagicLinkSent,
    Login,
    Error(Option<String>),
}

pub struct LoginComponent {
    email: NodeRef,
    error: Option<String>,
}

impl Component for LoginComponent {
    type Message = LoginMsg;
    type Properties = FormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: NodeRef::default(),
            error: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                        <button class="button is-link">{"Send Magic Link"}</button>
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
                ctx.link().send_future(async move {
                    let email = email_ref.cast::<HtmlInputElement>().unwrap().value();

                    #[derive(Serialize)]
                    struct Options {
                        #[serde(rename = "emailRedirectTo")]
                        email_redirect_to: String,
                    }
                    #[derive(Serialize)]
                    struct Credentials {
                        email: String,
                        options: Options,
                    }

                    let creds = Credentials {
                        email,
                        options: Options {
                            email_redirect_to: "recurr://magic_link".to_string(),
                        },
                    };

                    let res = client
                        .auth()
                        .sign_in_with_otp(
                            serde_wasm_bindgen::to_value(&creds)
                                .expect("Failed to serialize credentials"),
                        )
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

                            LoginMsg::MagicLinkSent
                        }
                        Err(e) => {
                            LoginMsg::Error(Some(e.as_string().expect("Failed to get string")))
                        }
                    }
                });
            }
            LoginMsg::Error(e) => self.error = e,
            LoginMsg::MagicLinkSent => {
                ctx.props().auth_cb.emit(super::AuthMessage::MagicLinkSent);
            }
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
