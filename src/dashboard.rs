use web_sys::MouseEvent;
use yew::{function_component, html, platform::spawn_local, use_context, Html};

use crate::context::SessionContext;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
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
        <div class="full-height columns m-0">
            <div class="column is-one-fifth has-background-info is-flex is-flex-direction-column">
                <div class="is-flex-grow-1">
                    {"Sidebar"}
                </div>

                <div class="is-flex is-justify-content-center">
                    <button onclick={signout} class="button is-danger">{"Signout"}</button>
                </div>
            </div>
            <div class="column">
                {"Main Area"}
            </div>
        </div>
    }
}
