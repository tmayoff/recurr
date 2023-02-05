mod login;
mod signup;

use login::LoginComponent;
use supabase_js_rs::SupabaseClient;
use yew::{function_component, html, use_context, Html, Properties};
use yew_hooks::{use_toggle, UseToggleHandle};

use crate::{auth::signup::SignupComponent, context::SessionContext};

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub toggle: UseToggleHandle<SignupSignin>,
    pub client: SupabaseClient,
}

#[derive(PartialEq)]
pub enum SignupSignin {
    Signup,
    Signin,
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
