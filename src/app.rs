use std::rc::Rc;
use supabase_js_rs::SupabaseClient;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct SessionContext {
    // supabase_client: SupabaseClient,
}

#[function_component(App)]
pub fn app() -> Html {
    let context = use_memo(|_| SessionContext {}, ());

    let signup = |event: SubmitEvent| {};

    html! {
        <ContextProvider<Rc<SessionContext>> context={context}>
            <main class="full-height">
                <div class="full-height is-flex is-justify-content-center is-align-content-center is-align-items-center">
                    <div class="has-shadow has-radius p-3">
                        // <h1 class="is-size-3">{"Sign In"}</h1>

                        // <form onsubmit={signup}>
                        //     <div class="field">
                        //         <label class="label">{"Email"}</label>
                        //         <div class="control">
                        //             <input class="input" type="email" placeholder="username"/>
                        //         </div>
                        //     </div>
                        //     <div class="field">
                        //         <label class="label">{"Password"}</label>
                        //         <div class="control">
                        //             <input class="input" type="password" placeholder="password"/>
                        //         </div>
                        //     </div>
                        //     <div class="field">
                        //         <div class="control">
                        //             <button class="button is-link">{"Login"}</button>
                        //         </div>
                        //     </div>
                        // </form>
                    </div>
                </div>

            // <div class="column">
            //     {"Main Area"}
            // </div>
            </main>
        </ContextProvider<Rc<SessionContext>>>
    }
}
