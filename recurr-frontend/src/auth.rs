use serde::Deserialize;
use supabase_js_rs::{Credentials, SupabaseClient};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlInputElement, MouseEvent, SubmitEvent};
use yew::{
    function_component, html, use_context, Callback, Component, Context, Html, NodeRef, Properties,
};
use yew_hooks::{use_toggle, UseToggleHandle};

use crate::context::SessionContext;

#[derive(PartialEq)]
enum SignupSignin {
    Signup,
    Signin,
}

#[derive(Properties, PartialEq)]
struct FormProps {
    toggle: UseToggleHandle<SignupSignin>,
    client: SupabaseClient,
}

enum ComponentMsg {
    LoggedIn,
    Login,
    Error(Option<String>),
}

struct LoginComponent {
    email: NodeRef,
    password: NodeRef,

    error: Option<String>,
}

impl Component for LoginComponent {
    type Message = ComponentMsg;
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
            ComponentMsg::Login
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
                <div class="field">
                    <div class="control">
                        <button class="button is-link">{"Login"}</button>
                    </div>
                </div>
                {
                    if let Some(e) = &self.error {
                        html!{
                            <div class="field">
                                <label class="label is-danger">{e}</label>
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }
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
            ComponentMsg::Login => {
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
                                return ComponentMsg::Error(Some(e.message));
                            }

                            ComponentMsg::LoggedIn
                        }
                        Err(e) => {
                            ComponentMsg::Error(Some(e.as_string().expect("Failed to get string")))
                        }
                    }
                });
            }
            ComponentMsg::Error(e) => self.error = e,
            ComponentMsg::LoggedIn => log::info!("Logged in"),
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

struct SignupComponent {
    email: NodeRef,
    password: NodeRef,
    confirm_password: NodeRef,
    confirm_pass_error: NodeRef,
}

impl Component for SignupComponent {
    type Message = ComponentMsg;
    type Properties = FormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: NodeRef::default(),
            password: NodeRef::default(),
            confirm_password: NodeRef::default(),
            confirm_pass_error: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let email_ref = self.email.clone();
        let pass_ref = self.password.clone();
        let conf_pass_ref = self.confirm_password.clone();
        let conf_pass_error_ref = self.confirm_pass_error.clone();

        let client = ctx.props().client.clone();
        let signup = move |event: SubmitEvent| {
            event.prevent_default();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = pass_ref.cast::<HtmlInputElement>().unwrap().value();
            let confirm_password = conf_pass_ref.cast::<HtmlInputElement>().unwrap().value();

            if password != confirm_password {
                conf_pass_error_ref
                    .cast::<HtmlElement>()
                    .unwrap()
                    .set_inner_text("Passwords must match");
                return;
            } else {
                conf_pass_error_ref
                    .cast::<HtmlElement>()
                    .unwrap()
                    .set_inner_text("");
            }

            if email.is_empty() || password.is_empty() || confirm_password.is_empty() {
                return;
            }

            let client = client.clone();

            spawn_local(async move {
                let _ = client.auth().sign_up(Credentials { email, password }).await;
            });
        };

        let toggle = ctx.props().toggle.clone();
        let toggle_form = { Callback::from(move |_| toggle.toggle()) };

        html! {
        <>
            <h1 class="is-size-3">{"Sign Up"}</h1>
            <form onsubmit={signup}>
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
                <div class="field">
                    <label class="label">{"Confirm Password"}</label>
                    <div class="control">
                        <input ref={self.confirm_password.clone()} class="input" type="password" placeholder="password"/>
                    </div>
                    <p ref={self.confirm_pass_error.clone()} class="help is-danger" >{"Passwords must match"}</p>
                </div>
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
}

#[function_component(Auth)]
pub fn auth() -> Html {
    let context = use_context::<SessionContext>().expect("Needs context");

    let signin_signup = use_toggle(SignupSignin::Signin, SignupSignin::Signup);

    let client = context
        .supabase_client
        .clone()
        .expect("Requires supabase client");

    html! {
        <div class="hero is-fullheight is-flex is-justify-content-center is-align-content-center is-align-items-center">
            <div class="has-shadow has-radius p-3">
                {
                    if (*signin_signup) == SignupSignin::Signin {
                       html!{ <LoginComponent {client} toggle={signin_signup} /> }
                    } else {
                        html!{ <SignupComponent {client} toggle={signin_signup}/> }
                    }
                }
            </div>
        </div>
    }
}
