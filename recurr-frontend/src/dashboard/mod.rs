use std::str::FromStr;
use std::string::ToString;

use crate::{
    context::{Session, SessionContext},
    dashboard::{
        accounts::AccountsView, budgets::BudgetsView, summary::SummaryView,
        transactions::TransactionsView,
    },
};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};
use yew::{
    function_component, html, platform::spawn_local, use_context, Callback, Component, Html,
    Properties, UseReducerHandle,
};

use self::transactions::Filter;

mod accounts;
mod budgets;
mod summary;
mod transactions;

#[derive(Debug, Deserialize, Display, Clone, PartialEq, EnumString, EnumIter, Serialize)]
pub enum DashboardTab {
    Summary,
    Budgets,
    Transaction(Filter),
    Accounts,
}

#[derive(Properties, PartialEq)]
struct SidebarProps {
    active_tab: DashboardTab,
    switch_tab: Callback<DashboardTab>,
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
        let tab_switch = props.switch_tab.clone();
        Callback::from(move |e: MouseEvent| {
            let target = e.target().expect("Event should come with a target");
            let target = target.unchecked_into::<HtmlElement>();
            let data = target.get_attribute("data").expect("Invalid Tab Button");
            tab_switch.emit(DashboardTab::from_str(&data).expect("Invalid Tab button"));
        })
    };

    html! {
        <aside class="menu p-3 has-background-primary is-flex is-flex-direction-column is-align-content-center">
            <div class="is-flex-grow-1 is-flex is-flex-direction-column">
                {
                    DashboardTab::iter().map(|tab| {
                        let tab_name = tab.to_string();
                        if tab == props.active_tab {
                            html!{<button class="button is-primary is-active" data={tab_name.clone()}>{tab_name}</button>}
                        } else {
                            html!{<button class="button is-primary" data={tab_name.clone()} onclick={switch_tabs.clone()}>{tab_name}</button>}
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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub context: UseReducerHandle<Session>,
}

pub enum Msg {
    SwitchTabs(DashboardTab),
}

pub struct Dashboard {
    active_tab: DashboardTab,
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let tab = LocalStorage::get("SavedTab").unwrap_or(DashboardTab::Summary);

        Self { active_tab: tab }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let context = &ctx.props().context;

        let active_tab = &self.active_tab;
        let switch_tab = ctx.link().callback(Msg::SwitchTabs);

        html! {
            <div class="full-height columns m-0">
                <Sidebar active_tab={active_tab.clone()} {switch_tab} />
                <div class="column has-background-light">
                    {
                        match &self.active_tab {
                            DashboardTab::Summary => html!{<SummaryView context={context.clone()} />},
                            DashboardTab::Budgets => html!{<BudgetsView context={context.clone()} />},
                            DashboardTab::Transaction(filter) => html!{<TransactionsView context={context.clone()} filter={filter.clone()}/>},
                            DashboardTab::Accounts => html!{<AccountsView />},
                        }
                    }
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchTabs(tab) => {
                LocalStorage::set("SavedTab", &tab).expect("Failed to save tab to local storage");
                self.active_tab = tab
            }
        }

        true
    }
}
