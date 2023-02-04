use recurr_core::Account;
use yew::{function_component, html, use_context, use_state, Html};
use yew_hooks::use_async;

use crate::{commands, context::SessionContext};

async fn get_balances(auth_key: &str, user_id: &str) -> Result<Vec<Account>, String> {
    commands::get_balances(auth_key, user_id).await
}

#[function_component(SummaryView)]
pub fn summary_view() -> Html {
    let context = use_context::<SessionContext>().expect("Requires context");
    let access_token = context
        .supabase_session
        .clone()
        .expect("Requires supabase session")
        .auth_key;

    let user_id = context
        .supabase_session
        .clone()
        .expect("Requires supabase session")
        .user
        .id;

    let balances = use_async(async move { get_balances(&access_token, &user_id).await });

    use_state(|| balances.run());

    html! {
        <div>
        {
            if balances.loading {
                html!{"Loading..."}
            } else {
                html!{}
            }
        }

        {
            if let Some(data) = &balances.data {
                data.clone().into_iter().map(|account| {
                    html!{
                        <div class="m-3 card">
                            <div class="card-header">
                                <h1 class="card-header-title">{account.name}</h1>
                            </div>

                            <div class="card-content">
                                {"$"} {account.balances.current}
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            } else {
                html!{""}
            }
        }
        </div>
    }
}
