use supabase_js_rs::Credentials;
use wasm_bindgen_futures::spawn_local;
use web_sys::SubmitEvent;
use yew::{function_component, html, Html};

#[function_component(Auth)]
pub fn auth() -> Html {
    let onsignup = |event: SubmitEvent| {
        spawn_local(async move {
            event.prevent_default();

            let client = supabase_js_rs::create_client(
                "https://linaejyblplchxcrusjy.supabase.co",
                "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI",
            );

            let email = String::from("tyler@tylermayoff.com");
            let password = String::from("password");

            let res = client.auth().sign_up(Credentials { email, password }).await;
            log::info!("{:?}", res);
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
