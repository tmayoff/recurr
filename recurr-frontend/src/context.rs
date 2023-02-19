use crate::supabase;
use std::rc::Rc;
use supabase_js_rs::SupabaseClient;
use yew::prelude::*;

pub enum ContextUpdate {
    Session(Option<supabase::Session>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub anon_key: String,
    pub supabase_client: SupabaseClient,
    pub supabase_session: Option<supabase::Session>,
}

impl Reducible for Session {
    type Action = ContextUpdate;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut s = (*self).clone();

        match action {
            ContextUpdate::Session(session) => s.supabase_session = session,
        }

        s.into()
    }
}

pub type SessionContext = UseReducerHandle<Session>;

#[derive(Properties, Debug, PartialEq)]
pub struct SessionProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn SessionProvider(props: &SessionProviderProps) -> Html {
    let auth_url = env!("SUPABASE_URL").to_string();
    let anon_key = env!("SUPABASE_KEY").to_string();
    let supabase_client = use_state(|| supabase_js_rs::create_client(&auth_url, &anon_key));

    let context = use_reducer(|| Session {
        supabase_client: (*supabase_client).clone(),
        supabase_session: None,
        anon_key: String::default(),
    });

    html! {
        <ContextProvider<SessionContext> context={context}>
            {props.children.clone()}
        </ContextProvider<SessionContext>>
    }
}
