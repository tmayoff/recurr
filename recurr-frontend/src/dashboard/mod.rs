use std::str::FromStr;

use crate::{
    context::{Session, SessionContext},
    dashboard::{accounts::AccountsView, summary::SummaryView, transactions::TransactionsView},
};
use strum::EnumString;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::{
    function_component, html, platform::spawn_local, use_context, use_state, Callback, Html,
    Properties, UseReducerHandle, UseStateHandle,
};

mod accounts;
mod summary;
mod transactions;

#[derive(Debug, PartialEq, EnumString)]
enum DashboardTab {
    Summary,
    Transaction,
    Accounts,
}

#[derive(Properties, PartialEq)]
struct SidebarProps {
    sidebar_state: UseStateHandle<DashboardTab>,
}

#[function_component(Sidebar)]
fn sidebar(props: &SidebarProps) -> Html {
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

            log::info!("{:?}", res);
            if let Err(e) = res {
                log::error!("{:?}", e);
            }
        });
    };

    let switch_tabs = {
        let tab = props.sidebar_state.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().expect("Event should come with a target");
            let target = target.unchecked_into::<HtmlElement>();
            let data = target.get_attribute("data").expect("Invalid Tab Button");
            tab.set(DashboardTab::from_str(&data).expect("Invalid Tab button"));
        })
    };

    struct TabButton {
        active: bool,
        name: String,
        tab: DashboardTab,
    }

    let mut tab_buttons = vec![
        TabButton {
            active: false,
            name: "Summary".to_string(),
            tab: DashboardTab::Summary,
        },
        TabButton {
            active: true,
            name: "Transaction".to_string(),
            tab: DashboardTab::Transaction,
        },
        TabButton {
            active: false,
            name: "Accounts".to_string(),
            tab: DashboardTab::Accounts,
        },
    ];

    let tabs = props.sidebar_state.clone();
    for button in &mut tab_buttons {
        button.active = button.tab == *tabs;
    }

    html! {
        <div class="column is-one-fifth has-background-info is-flex is-flex-direction-column">
            <div class="is-flex-grow-1 is-flex is-flex-direction-column">
                {
                    tab_buttons.into_iter().map(|tab| {
                        if tab.active {
                            html!{<button class="button is-info is-active" data={format!("{:?}", tab.tab)}>{tab.name}</button>}
                        } else {
                            html!{<button class="button is-info" data={format!("{:?}", tab.tab)} onclick={switch_tabs.clone()}>{tab.name}</button>}
                        }
                    }).collect::<Html>()
                }
            </div>

            <div class="is-flex is-justify-content-center">
                <button onclick={signout} class="button is-danger">{"Signout"}</button>
            </div>
        </div>
    }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let sidebar_state = use_state(|| DashboardTab::Summary);
    let context = use_context::<UseReducerHandle<Session>>();

    html! {
        <div class="full-height columns m-0">
            <Sidebar sidebar_state={sidebar_state.clone()} />
            <div class="column">
                {
                    match *sidebar_state {
                        DashboardTab::Summary => html!{<SummaryView />},
                        DashboardTab::Transaction => {
                                if let Some(session) = context {
                                    html!{<TransactionsView {session}/>}
                                }  else {
                                    html!{""}
                                }
                            },
                        DashboardTab::Accounts => html!{<AccountsView />},
                    }
                }
            </div>
        </div>
    }
}
