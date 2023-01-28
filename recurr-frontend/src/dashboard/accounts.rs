use yew::{function_component, html, use_context, use_state, Html};
use yew_hooks::prelude::*;

use crate::{commands::get_all_accounts, context::SessionContext, plaid::Link};

#[function_component(AccountsView)]
pub fn accounts_view() -> Html {
    html! {
        <div>
            <div class="is-flex is-flex-direction-row is-justify-content-space-around is-align-items-center">
                <h1 class="is-size-3">{"All Accounts"}</h1>
                <Link />
            </div>
            <Accounts />
        </div>
    }
}

#[function_component(Accounts)]
pub fn accounts() -> Html {
    let context = use_context::<SessionContext>().expect("Requires context");

    let session = context
        .supabase_session
        .as_ref()
        .expect("Requires supabase session");
    let user_id = session.user.id.clone();
    let access_token = session.access_token.clone();

    let accounts = use_async(async move {
        let accounts = get_all_accounts(&access_token, &user_id).await;
        log::info!("{:?}", &accounts);
        accounts
    });

    use_state(|| accounts.run());

    html! {
        <div class="is-flex p-2">
            {
                if accounts.loading {
                    html!{"Loading Accounts"}
                } else {
                    html!{}
                }
            }
            {
                if let Some(data) = &accounts.data {
                    data.clone().into_iter().map(|account| {
                        html!{
                            <div class="m-3 card">
                                <div class="card-content">
                                    <h1 class="is-size-5">{account.official_name}</h1>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                } else {
                    html!{}
                }
            }
        </div>
    }
}
