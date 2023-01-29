use crate::{
    context::SessionContext,
    dashboard::{accounts::AccountsView, summary::SummaryView},
};
use web_sys::MouseEvent;
use yew::{
    classes, function_component, html, platform::spawn_local, use_context, use_state, Callback,
    Html, Properties, UseStateHandle,
};

mod accounts;
mod summary;

#[derive(PartialEq)]
enum DashboardTab {
    Summary,
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
            if let Err(e) = res {
                log::error!("{:?}", e);
            }
        });
    };

    let switch_tabs = {
        let tab = props.sidebar_state.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().expect("Event should come with a target");

            log::info!("Switching tabs {:?}", target);
            // tab.set()
        })
    };

    let base_classes = vec!["button", "is-info"];

    html! {
        <div class="column is-one-fifth has-background-info is-flex is-flex-direction-column">
            <div class="is-flex-grow-1 is-flex is-flex-direction-column">
                <button class="button is-info is-active" onclick={switch_tabs}>{"Summary"}</button>
                <button class={classes!(base_classes)}>{"Accounts"}</button>
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
    let sidebar_state = use_state(|| DashboardTab::Summary);

    html! {
        <div class="full-height columns m-0">
            <Sidebar sidebar_state={sidebar_state.clone()} />
            <div class="column">
                {
                    if (*sidebar_state) == DashboardTab::Summary {
                        html!{<SummaryView />}
                    } else if (*sidebar_state) == DashboardTab::Accounts {
                        html!{<AccountsView />}
                    } else {
                        html!{}
                    }
                }
            </div>
        </div>
    }
}
