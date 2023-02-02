use recurr_core::Account;
use yew::{function_component, html, use_context, use_state, Html};
use yew_hooks::use_async;

use crate::{commands, context::SessionContext};

async fn get_balances(access_token: &str, user_id: &str) -> Result<Vec<Account>, String> {
    let accounts = commands::get_all_accounts(access_token, user_id).await?;

    let mut account_ids = Vec::new();
    for account in accounts {
        account_ids.extend(account.1.clone());
    }

    let account_ids: Vec<String> = account_ids.into_iter().map(|a| a.account_id).collect();
    commands::get_balances(access_token, "", user_id).await;

    Ok(Vec::new())
}

#[function_component(SummaryView)]
pub fn summary_view() -> Html {
    let context = use_context::<SessionContext>().expect("Requires context");
    let access_token = context
        .supabase_session
        .clone()
        .expect("Requires supabase session")
        .access_token;

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
                html!{format!("{data:?}")}
            } else {
                html!{""}
            }
        }
        </div>
    }
}
