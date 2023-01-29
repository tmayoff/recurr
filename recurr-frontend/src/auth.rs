use supabase_js_rs::Credentials;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent, SubmitEvent};
use yew::{function_component, html, use_context, use_state, Callback, Html};
use yew_hooks::use_toggle;

use crate::context::SessionContext;

#[derive(PartialEq)]
enum SignupSignin {
    Signup,
    Signin,
}

#[function_component(Auth)]
pub fn auth() -> Html {
    // TODO this function could do with a cleanup
    let context = use_context::<SessionContext>().expect("No context found");

    let signin_signup = use_toggle(SignupSignin::Signup, SignupSignin::Signin);

    let email_input_value_handle = use_state(String::default);
    let email_input_value = (*email_input_value_handle).clone();
    let email_input_onchange: Callback<InputEvent> = {
        let email_input_value_handle = email_input_value_handle;

        Callback::from(move |event: InputEvent| {
            let target = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                email_input_value_handle.set(input.value());
            }
        })
    };

    let password_input_value_handle = use_state(String::default);
    let password_input_value = (*password_input_value_handle).clone();

    let password_input_onchange: Callback<InputEvent> = {
        let password_input_value_handle = password_input_value_handle;

        Callback::from(move |event: InputEvent| {
            let target = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                password_input_value_handle.set(input.value());
            }
        })
    };

    let signup = move |event: SubmitEvent| {};

    let signin = move |event: SubmitEvent| {
        event.prevent_default();

        let context = context.clone();
        let password = email_input_value.clone();
        let email = email_input_value.clone();

        spawn_local(async move {
            let _ = context
                .supabase_client
                .clone()
                .expect("Must have supabase client")
                .auth()
                .sign_in_with_password(Credentials { email, password })
                .await;
        });
    };

    html! {
        <div class="hero is-fullheight is-flex is-justify-content-center is-align-content-center is-align-items-center">
            <div class="has-shadow has-radius p-3">

                {
                    if (*signin_signup) == SignupSignin::Signin {
                       html!{
                        <>
                            <h1 class="is-size-3">{"Sign In"}</h1>
                            <form onsubmit={signin}>
                                <div class="field">
                                    <label class="label">{"Email"}</label>
                                    <div class="control">
                                        <input oninput={email_input_onchange} class="input" type="email" placeholder="username"/>
                                    </div>
                                </div>
                                <div class="field">
                                    <label class="label">{"Password"}</label>
                                    <div class="control">
                                        <input oninput={password_input_onchange} class="input" type="password" placeholder="password" value={password_input_value}/>
                                    </div>
                                </div>
                                <div class="field">
                                    <div class="control">
                                        <button class="button is-link">{"Login"}</button>
                                    </div>
                                </div>
                            </form>
                        </>
                        }
                    } else {
                        html!{
                            <>
                                <h1 class="is-size-3">{"Sign Up"}</h1>
                                <form onsubmit={signin}>
                                    <div class="field">
                                        <label class="label">{"Email"}</label>
                                        <div class="control">
                                            <input oninput={email_input_onchange} class="input" type="email" placeholder="username"/>
                                        </div>
                                    </div>
                                    <div class="field">
                                        <label class="label">{"Password"}</label>
                                        <div class="control">
                                            <input oninput={password_input_onchange} class="input" type="password" placeholder="password" value={password_input_value}/>
                                        </div>
                                    </div>
                                    <div class="field">
                                        <div class="control">
                                            <button class="button is-link">{"Login"}</button>
                                        </div>
                                    </div>
                                </form>
                            </>
                            }
                    }
                }

            </div>
        </div>
    }
}
