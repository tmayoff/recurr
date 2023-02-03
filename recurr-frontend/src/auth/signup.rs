use serde::Deserialize;
use supabase_js_rs::{Credentials, SupabaseClient};
use web_sys::{HtmlElement, HtmlInputElement, SubmitEvent};
use yew::{html, platform::spawn_local, Callback, Component, Context, Html, NodeRef};

use super::FormProps;

pub enum SignupMsg {
    Signup,
    SignedUp,
    Error(Option<String>),
}

pub struct SignupComponent {
    email: NodeRef,
    password: NodeRef,
    confirm_password: NodeRef,

    error: Option<String>,
}

// impl SignupComponent {
//     async fn signup(&self, client: &SupabaseClient) -> SignupMsg {
//         let email_ref = self.email.clone();
//         let pass_ref = self.password.clone();
//         let conf_pass_ref = self.confirm_password.clone();

//         let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
//         let password = pass_ref.cast::<HtmlInputElement>().unwrap().value();
//         let confirm_password = conf_pass_ref.cast::<HtmlInputElement>().unwrap().value();

//         if password != confirm_password {
//             return SignupMsg::Error(Some("Passwords match".to_string()));
//         }

//         let client = client.clone();

//         let _ = client.auth().sign_up(Credentials { email, password }).await;

//         SignupMsg::SignedUp
//     }
// }

impl Component for SignupComponent {
    type Message = SignupMsg;
    type Properties = FormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: NodeRef::default(),
            password: NodeRef::default(),
            confirm_password: NodeRef::default(),
            error: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let signup = move |event: SubmitEvent| {
        //     event.prevent_default();
        //     let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
        //     let password = pass_ref.cast::<HtmlInputElement>().unwrap().value();
        //     let confirm_password = conf_pass_ref.cast::<HtmlInputElement>().unwrap().value();

        //     if password != confirm_password {
        //         conf_pass_error_ref
        //             .cast::<HtmlElement>()
        //             .unwrap()
        //             .set_inner_text("Passwords must match");
        //         return;
        //     } else {
        //         conf_pass_error_ref
        //             .cast::<HtmlElement>()
        //             .unwrap()
        //             .set_inner_text("");
        //     }

        //     if email.is_empty() || password.is_empty() || confirm_password.is_empty() {
        //         return;
        //     }

        //     let client = client.clone();

        //     spawn_local(async move {
        //         let _ = client.auth().sign_up(Credentials { email, password }).await;
        //     });
        // };

        let toggle = ctx.props().toggle.clone();
        let toggle_form = { Callback::from(move |_| toggle.toggle()) };

        let signup = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            SignupMsg::Signup
        });

        html! {
        <>
            <h1 class="is-size-3">{"Sign Up"}</h1>
            <form onsubmit={signup}>
                <div class="field">
                    <label class="label">{"Email"}</label>
                    <div class="control">
                        <input ref={self.email.clone()} class="input" type="email" placeholder="username" required=true/>
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Password"}</label>
                    <div class="control">
                        <input ref={self.password.clone()} class="input" type="password" placeholder="password" required=true/>
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Confirm Password"}</label>
                    <div class="control">
                        <input ref={self.confirm_password.clone()} class="input" type="password" placeholder="password" required=true/>
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
                        <button class="button is-link">{"Sign Up"}</button>
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
            SignupMsg::Error(e) => self.error = e,
            SignupMsg::Signup => {
                let client = ctx.props().client.clone();
                let email_ref = self.email.clone();
                let pass_ref = self.password.clone();
                let conf_pass_ref = self.confirm_password.clone();

                ctx.link().send_future(async move {
                    let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
                    let password = pass_ref.cast::<HtmlInputElement>().unwrap().value();
                    let confirm_password =
                        conf_pass_ref.cast::<HtmlInputElement>().unwrap().value();

                    if password != confirm_password {
                        return SignupMsg::Error(Some("Passwords match".to_string()));
                    }

                    let client = client.clone();

                    let res = client.auth().sign_up(Credentials { email, password }).await;

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
                                return SignupMsg::Error(Some(e.message));
                            }

                            SignupMsg::SignedUp
                        }
                        Err(e) => {
                            SignupMsg::Error(Some(e.as_string().expect("Failed to get string")))
                        }
                    }
                });
            }
            SignupMsg::SignedUp => log::info!("Signed up"),
        }

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
