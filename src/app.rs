use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="columns full-height">
            <div class="column is-one-fifth has-background-info is-flex is-flex-direction-column">
                <div class="is-flex-grow-1">{"Sidebar"}</div>

                <div clas="is-align-self-flex-end">
                    {"Tyler Mayoff"}
                </div>
            </div>

            <div class="column">
                {"Main Area"}
            </div>
        </main>
    }
}
