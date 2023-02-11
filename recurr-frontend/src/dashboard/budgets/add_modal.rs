use recurr_core::{Category, SchemaBudget};
use web_sys::{HtmlElement, HtmlInputElement, SubmitEvent};
use yew::{html, Component, Context, Html, NodeRef, Properties, UseReducerHandle};

use crate::{commands, context::Session, supabase::get_supbase_client};

pub enum Msg {
    Error(String),
    GotCategories(Vec<Category>),
    OpenModal,
    CloseModal,
    Submit,
    Submitted,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub session: UseReducerHandle<Session>,
}

pub struct Modal {
    modal_ref: NodeRef,
    categories: Vec<Category>,

    category_ref: NodeRef,
    amount_ref: NodeRef,
}

impl Modal {
    async fn get_categories() -> Msg {
        let categories = commands::get_categories()
            .await
            .expect("Failed to get categories");
        //TODO Would be good to either sort this or group it
        Msg::GotCategories(categories)
    }

    fn set_modal_class(&self, class: &str) {
        let elem = self.modal_ref.cast::<HtmlElement>();
        if let Some(elem) = elem {
            elem.set_class_name(class);
        }
    }
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(Self::get_categories());

        Self {
            modal_ref: NodeRef::default(),
            categories: Vec::new(),

            category_ref: NodeRef::default(),
            amount_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_modal = ctx.link().callback(|_| Msg::CloseModal);
        let open_modal = ctx.link().callback(|_| Msg::OpenModal);
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::Submit
        });

        html! {
            <>
            <div class="modal" ref={self.modal_ref.clone()}>
                <div class="modal-background" onclick={close_modal.clone()}></div>
                <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{"Add budget"}</p>
                    <button class="delete" aria-label="close" onclick={close_modal.clone()}></button>
                </header>
                {
                    if !self.categories.is_empty() {
                        html!{
                            <>
                            <form onsubmit={on_submit}>
                            <section class="modal-card-body">
                                    <div class="select is-info">
                                        <select placeholder="Choose a category" ref={self.category_ref.clone()}>
                                            {
                                                self.categories.clone().iter().map(|c| {
                                                    html!{<option>{c.hierarchy.last()}</option>}
                                                }).collect::<Html>()
                                            }
                                        </select>
                                    </div>

                                    <div class="field">
                                        <label class="label">{"How much"}</label>
                                        <div class="control">
                                            <input class="input is-success" type="number" value="0" ref={self.amount_ref.clone()}/>
                                        </div>
                                    </div>
                                    </section>
                                <footer class="modal-card-foot">
                                    <button class="button" onclick={close_modal.clone()}>{"Cancel"}</button>
                                    <button class="button is-success" type="submit">{"Save"}</button>

                                </footer>
                            </form>
                            </>
                        }
                    } else {
                        html!{}
                    }
                }
                </div>
            </div>

            <button class="button is-success" onclick={open_modal}>{"Add a budget"}</button>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotCategories(categories) => self.categories = categories,
            Msg::OpenModal => self.set_modal_class("modal is-active"),
            Msg::CloseModal => self.set_modal_class("modal"),
            Msg::Submit => {
                let amount = self
                    .amount_ref
                    .cast::<HtmlInputElement>()
                    .expect("Amount ref not an input element")
                    .value()
                    .parse()
                    .expect("Failed to parse amount");

                let category = self
                    .category_ref
                    .cast::<HtmlInputElement>()
                    .expect("Category ref not an input element")
                    .value();

                let session = ctx
                    .props()
                    .session
                    .clone()
                    .supabase_session
                    .clone()
                    .expect("Needs session");
                let auth_key = session.auth_key;
                let user_id = session.user.id;

                let db_client = get_supbase_client();

                let schema = SchemaBudget {
                    user_id,
                    category,
                    max: amount,
                }
                .to_string()
                .expect("Failed to serialize");

                ctx.link().send_future(async move {
                    let res = db_client
                        .from("budgets")
                        .auth(&auth_key)
                        .insert(schema)
                        .execute()
                        .await;

                    match res {
                        Ok(r) => {
                            if r.status().is_success() {
                                Msg::Submitted
                            } else {
                                Msg::Error(r.status().to_string())
                            }
                        }
                        Err(e) => Msg::Error(e.to_string()),
                    }
                });
            }
            Msg::Submitted => ctx.link().send_message(Msg::CloseModal),
            Msg::Error(e) => log::error!("{e}"),
        }

        true
    }
}
