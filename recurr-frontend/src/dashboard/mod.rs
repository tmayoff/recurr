use crate::{
    context::SessionContext,
    dashboard::{
        accounts::AccountsView, budgets::BudgetsView, summary::SummaryView,
        transactions::TransactionsView,
    },
};
use web_sys::MouseEvent;
use yew::{function_component, html, platform::spawn_local, use_context, Html};
use yew_router::{prelude::use_route, BrowserRouter, Routable, Switch};

mod accounts;
mod budgets;
mod summary;
mod transactions;

#[function_component(Sidebar)]
fn sidebar() -> Html {
    let context = use_context::<SessionContext>().unwrap();
    let use_context = context;

    let signout = move |_: MouseEvent| {
        let use_context = use_context.clone();
        spawn_local(async move {
            let res = use_context
                .supabase_client
                .clone()
                .expect("Must have supabase client")
                .auth()
                .sign_out()
                .await;

            if let Err(e) = res {
                log::error!("{:?}", e);
            }
        });
    };

    // TODO this could be cleaned up probably
    struct TabButton {
        route: Route,
        endpoint: String,
    }

    let tab_buttons = vec![
        TabButton {
            route: Route::Summary,
            endpoint: "/".to_string(),
        },
        TabButton {
            route: Route::Budgets,
            endpoint: "/budgets".to_string(),
        },
        TabButton {
            route: Route::Transactions,
            endpoint: "/transactions".to_string(),
        },
        TabButton {
            route: Route::Accounts,
            endpoint: "/accounts".to_string(),
        },
    ];

    let route: Route = use_route().unwrap();

    html! {
        <aside class="menu p-3 has-background-primary is-flex is-flex-direction-column is-align-content-center">
            <div class="is-flex-grow-1 is-flex is-flex-direction-column">
                {
                    tab_buttons.into_iter().map(|tab| {
                        if tab.route == route {
                            html!{<a href={tab.endpoint} class="button is-primary is-active">{format!("{:?}", tab.route)}</a>}
                        } else {
                            html!{<a href={tab.endpoint} class="button is-primary">{format!("{:?}", tab.route)}</a>}
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="is-flex is-justify-content-center">
                <button onclick={signout} class="button is-danger">{"Signout"}</button>
            </div>
        </aside>
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Summary,
    #[at("/budgets")]
    Budgets,
    #[at("/transactions")]
    Transactions,
    #[at("/accounts")]
    Accounts,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Summary => html! {
            <>
            <Sidebar />
            <SummaryView />
            </>
        },
        Route::Budgets => html! {
            <>
            <Sidebar />
            <BudgetsView />
            </>
        },
        Route::Transactions => html! {
            <>
            <Sidebar />
            <TransactionsView />
            </>
        },
        Route::Accounts => html! {
            <>
            <Sidebar />
            <AccountsView />
            </>
        },
    }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="full-height columns m-0">
            <BrowserRouter>
                <Switch <Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}
