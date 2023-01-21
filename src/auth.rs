use wasm_bindgen_futures::spawn_local;
use web_sys::SubmitEvent;
use yew::{function_component, html, Html};

#[function_component(Auth)]
pub fn auth() -> Html {
    let onsignup = |event: SubmitEvent| {
        spawn_local(async move {
            event.prevent_default();

            // let res = session.sign_up("tyler@tylermayoff.com", "password").await;
            // log::info!("{:?}", res)
        })
    };

    html! {
        <div>
            <form onsubmit={onsignup}>
                <input />
                <input />
                <button>{"Sign Up"}</button>
            </form>
        </div>
    }
}
