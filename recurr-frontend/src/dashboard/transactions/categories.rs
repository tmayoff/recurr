use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Component, Html, Properties};

use crate::commands;

#[derive(Debug, PartialEq, Eq, Clone)]
struct CategoryComp {
    category: String,
    children: Vec<CategoryComp>,
}

#[derive(Default)]
pub struct Categories {
    categories: Vec<CategoryComp>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub show: bool,

    pub on_toggle: Callback<MouseEvent>,
}

pub enum Messages {
    GetCategories,
    GotCategories(Vec<CategoryComp>),
}

impl Component for Categories {
    type Message = Messages;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_message(Messages::GetCategories);

        Self::default()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let close_modal = ctx.props().on_toggle.clone();

        if ctx.props().show {
            html! {
            <>
                <div class="modal is-active">
                    <div class="modal-background" onclick={close_modal.clone()}></div>

                    <div class="modal-card">
                        <header class="modal-card-head">
                            <h1 class="modal-card-title">{"Categories"}</h1>
                        </header>

                        <section class="modal-card-body">
                            {self.categories.iter().map(|c| html!{<ul class="ml-4 list"> <Category category={c.clone()}/> </ul>}).collect::<Html>()}
                        </section>

                        <footer class="modal-card-foot"> </footer>
                    </div>
                </div>
            </>
            }
        } else {
            html! {}
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Messages::GetCategories => {
                ctx.link().send_future(async {
                    let categories = commands::get_categories().await.unwrap();

                    let mut c: Vec<CategoryComp> = Vec::new();
                    for cat in categories {
                        recursively_update(&mut c, &cat.hierarchy);
                    }

                    Messages::GotCategories(c)
                });
            }
            Messages::GotCategories(c) => {
                log::info!("{:?}", c);
                self.categories = c;
            }
        }

        true
    }
}

#[derive(Properties, PartialEq)]
struct CatProps {
    category: CategoryComp,
}

#[function_component]
fn Category(props: &CatProps) -> Html {
    let name = props.category.category.clone();

    html! {
            <li class="list-item">
                {name}
                    if !props.category.children.is_empty() {
                        <ul class="list ml-4">
                            {props.category.children.iter().map(|c| html!{<Category category={c.clone()}/>}).collect::<Html>()}
                        </ul>
                    }
            </li>
    }
}

fn recursively_update(categories: &mut Vec<CategoryComp>, category: &Vec<String>) {
    let first = category.first();
    if first.is_none() {
        return;
    }

    let first = first.unwrap();

    for c in &mut *categories {
        if &c.category == first {
            let mut cat = category.clone();
            cat.remove(0);
            recursively_update(&mut c.children, &cat);
            return;
        }
    }

    categories.push(CategoryComp {
        category: first.to_owned(),
        children: Vec::new(),
    });
}
