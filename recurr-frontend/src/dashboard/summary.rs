use yew::{function_component, html, use_context, use_state, Html};
use yew_hooks::use_async;

use crate::{commands::get_balances, context::SessionContext};

#[function_component(SummaryView)]
pub fn summary_view() -> Html {
    let context = use_context::<SessionContext>().expect("Requires context");
    let access_token = context
        .supabase_session
        .clone()
        .expect("Requires supabase session")
        .access_token;

    let balances = use_async(async move { get_balances(&access_token, "").await });

    use_state(|| balances.run());

    html! {
        <div>
        {
            if balances.loading {
                html!{"Loading..."}
            } else {
                html!{{"Loaded"}}
            }
        }
        </div>
    }
}
