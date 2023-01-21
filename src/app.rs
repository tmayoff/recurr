use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="full-height">
            <div class="full-height is-flex is-justify-content-center is-align-content-center is-align-items-center">
                <div class="has-shadow has-radius p-3">
                    <h1>{"Welcome"}</h1>

                    <form>
                        <div class="field">
                            <div class="control">
                                <input placeholder="username"/>
                            </div>
                        </div>
                        <div class="field">
                            <div class="control">
                                <input placeholder="password"/>
                            </div>
                        </div>
                        <div class="field">
                            <div class="control">
                                <button>{"Login"}</button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>

            // <div class="columns">
            //     <div class="column is-one-fifth has-background-info is-flex is-flex-direction-columns">
            //         <div></div>
            //         <div class="is-align-self-flex-end">
            //             {"Bottom Text"}
            //         </div>
            //     </div>

            //     <div></div>
            // </div>
        </main>
    }
}
