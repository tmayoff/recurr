use recurr_core::{get_supbase_client, Category, SchemaBudget};
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties, UseReducerHandle};

use crate::{commands, context::Session};

#[derive(Debug, PartialEq)]
pub enum ModalMsg {
    Close,
    Save,
}

pub enum Msg {
    Error(String),
    GotCategories(Vec<Category>),
    CloseModal,
    Delete,
    Submit,
    Submitted,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_change: Callback<ModalMsg>,
    pub session: UseReducerHandle<Session>,
    pub show: bool,

    pub detail: Option<SchemaBudget>,
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
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::Submit
        });

        let delete = ctx.link().callback(|_| Msg::Delete);

        if ctx.props().show {
            html! {
                <>
                <div class="modal is-active" ref={self.modal_ref.clone()}>
                    <div class="modal-background" onclick={close_modal.clone()}></div>
                    <div class="modal-card">
                    <header class="modal-card-head">
                        {
                        if ctx.props().detail.is_some() {
                            html!{<p class="modal-card-title">{"Edit budget"}</p>}
                        }else{
                            html!{<p class="modal-card-title">{"Add budget"}</p>}
                        }
                        }
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
                                                        let cat = c.hierarchy.last().expect("Requires an option").clone();
                                                        let selected = ctx.props().detail.clone().map_or(false, |d| d.category_id == cat);
                                                        html!{<option {selected}>{cat}</option>}
                                                    }).collect::<Html>()
                                                }
                                            </select>
                                        </div>

                                        <div class="field">
                                            <label class="label">{"How much"}</label>
                                            <div class="control">
                                                <input class="input is-success" type="number" value={ctx.props().detail.clone().map_or(0.0, |d| d.max).to_string()} ref={self.amount_ref.clone()}/>
                                            </div>
                                        </div>
                                    </section>
                                    <footer class="modal-card-foot">
                                        <button class="button" onclick={close_modal.clone()}>{"Cancel"}</button>
                                        <button class="button is-success" type="submit">{"Save"}</button>
                                        if ctx.props().detail.is_some() {
                                            <button class="button is-danger" onclick={delete} type="button">{"Delete"}</button>
                                        }
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
                </>
            }
        } else {
            html! {}
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotCategories(categories) => self.categories = categories,
            Msg::CloseModal => ctx.props().on_change.emit(ModalMsg::Close),
            Msg::Submit => {
                let amount = self
                    .amount_ref
                    .cast::<HtmlInputElement>()
                    .expect("Amount ref not an input element")
                    .value()
                    .parse()
                    .expect("Failed to parse amount");

                let category_id = self
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
                    category_id,
                    max: amount,
                }
                .to_string()
                .expect("Failed to serialize");

                ctx.link().send_future(async move {
                    let res = db_client
                        .from("budgets")
                        .auth(&auth_key)
                        .upsert(schema)
                        .execute()
                        .await
                        .map(|r| r.error_for_status())
                        .flatten();

                    match res {
                        Ok(r) => Msg::Submitted,
                        Err(e) => Msg::Error(e.to_string()),
                    }
                });
            }
            Msg::Delete => {
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

                let category = self
                    .category_ref
                    .cast::<HtmlInputElement>()
                    .expect("Category ref not an input element")
                    .value();

                ctx.link().send_future(async move {
                    let res = db_client
                        .from("budgets")
                        .auth(&auth_key)
                        .eq("user_id", user_id)
                        .eq("category", category)
                        .delete()
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
            Msg::Submitted => {
                ctx.link().send_message(Msg::CloseModal);
                ctx.props().on_change.emit(ModalMsg::Save);
            }
            Msg::Error(e) => log::error!("{e}"),
        }

        true
    }
}
