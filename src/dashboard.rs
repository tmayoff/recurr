use crate::{context::SessionContext, plaid::Link};
use web_sys::MouseEvent;
use yew::{function_component, html, platform::spawn_local, use_context, Html};

#[function_component(Accounts)]
fn accounts() -> Html {
    html! {
        <div>
            <Link />
        </div>
    }
}

#[function_component(Sidebar)]
fn sidebar() -> Html {
    let context = use_context::<SessionContext>().unwrap();
    let use_context = context.clone();

    let signout = move |_: MouseEvent| {
        let use_context = use_context.clone();
        spawn_local(async move {
            let res = use_context.supabase_client.auth().sign_out().await;
            if let Err(e) = res {
                log::error!("{:?}", e);
            }
        });
    };

    html! {
        <div class="column is-one-fifth has-background-info is-flex is-flex-direction-column">
            <div class="is-flex-grow-1 is-flex is-flex-direction-column">
                <button class="button is-info">{"Dashboard"}</button>
                <button class="button is-info is-active">{"Accounts"}</button>
                <button class="button is-info">{"Dashboard"}</button>
                <button class="button is-info">{"Dashboard"}</button>
            </div>

            <div class="is-flex is-justify-content-center">
                <button onclick={signout} class="button is-danger">{"Signout"}</button>
            </div>
        </div>
    }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="full-height columns m-0">
            <Sidebar />
            <div class="column">
                <Accounts />
            </div>
        </div>
    }
}