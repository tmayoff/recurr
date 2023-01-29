use supabase_js_rs::Credentials;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::{
    function_component, html, Callback, Component, Context, ContextHandle, Html, NodeRef,
    Properties,
};
use yew_hooks::{use_toggle, UseToggleHandle};

use crate::context::Session;

#[derive(PartialEq)]
enum SignupSignin {
    Signup,
    Signin,
}

#[derive(Properties, PartialEq)]
struct FormProps {
    toggle: UseToggleHandle<SignupSignin>,
}

enum ComponentMsg {
    SessionContextUpdate(Session),
}

struct LoginComponent {
    context: Session,
    _context_listener: ContextHandle<Session>,

    email: NodeRef,
    password: NodeRef,
}

impl Component for LoginComponent {
    type Message = ComponentMsg;
    type Properties = FormProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (session, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(ComponentMsg::SessionContextUpdate))
            .expect("No message context provided");

        Self {
            context: session,
            _context_listener,
            email: NodeRef::default(),
            password: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMsg::SessionContextUpdate(session) => {
                self.context = session;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let email_ref = self.email.clone();
        let pass_ref = self.password.clone();

        let client = self
            .context
            .supabase_client
            .clone()
            .expect("Must have supabase client already");

        let signin = move |event: SubmitEvent| {
            event.prevent_default();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = pass_ref.cast::<HtmlInputElement>().unwrap().value();

            let client = client.clone();

            spawn_local(async move {
                let _ = client
                    .auth()
                    .sign_in_with_password(Credentials { email, password })
                    .await;
            });
        };

        let toggle = ctx.props().toggle.clone();
        let toggle_form = { Callback::from(move |_| toggle.toggle()) };

        html! {
        <>
            <h1 class="is-size-3">{"Sign In"}</h1>
            <form onsubmit={signin}>
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

struct SignupComponent {
    context: Session,
    _context_listener: ContextHandle<Session>,

    email: NodeRef,
    password: NodeRef,
    confirm_password: NodeRef,
}

impl Component for SignupComponent {
    type Message = ComponentMsg;
    type Properties = FormProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (session, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(ComponentMsg::SessionContextUpdate))
            .expect("No message context provided");

        Self {
            context: session,
            _context_listener,
            email: NodeRef::default(),
            password: NodeRef::default(),
            confirm_password: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let email_ref = self.email.clone();
        let pass_ref = self.password.clone();
        let conf_pass_ref = self.confirm_password.clone();

        let client = self
            .context
            .supabase_client
            .clone()
            .expect("Must have supabase client already");

        let signup = move |event: SubmitEvent| {
            event.prevent_default();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = pass_ref.cast::<HtmlInputElement>().unwrap().value();
            let confirm_password = conf_pass_ref.cast::<HtmlInputElement>().unwrap().value();

            // TODO Check password

            let client = client.clone();

            spawn_local(async move {
                let _ = client
                    .auth()
                    .sign_in_with_password(Credentials { email, password })
                    .await;
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
}

#[function_component(Auth)]
pub fn auth() -> Html {
    let signin_signup = use_toggle(SignupSignin::Signin, SignupSignin::Signup);

    html! {
        <div class="hero is-fullheight is-flex is-justify-content-center is-align-content-center is-align-items-center">
            <div class="has-shadow has-radius p-3">

                {
                    if (*signin_signup) == SignupSignin::Signin {
                       html!{ <LoginComponent toggle={signin_signup}/> }
                    } else {
                        html!{ <SignupComponent toggle={signin_signup}/> }
                    }
                }

            </div>
        </div>
    }
}
