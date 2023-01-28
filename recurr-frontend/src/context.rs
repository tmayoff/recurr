use crate::supabase;
use std::rc::Rc;
use supabase_js_rs::SupabaseClient;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub anon_key: String,
    pub supabase_client: Option<SupabaseClient>,
    pub supabase_session: Option<supabase::Session>,
}

impl Reducible for Session {
    type Action = (Option<supabase::Session>, Option<SupabaseClient>);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut s = Session {
            supabase_client: self.supabase_client.clone(),
            supabase_session: self.supabase_session.clone(),
            anon_key: String::default(),
        };

        // We don't want to overwrite either session or client if the other isn't provided in the call
        if let Some(session) = action.0 {
            s.supabase_session = Some(session);
        }

        if let Some(client) = action.1 {
            s.supabase_client = Some(client);
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
    let context = use_reducer(|| Session {
        supabase_client: None,
        supabase_session: None,
        anon_key: String::default(),
    });

    html! {
        <ContextProvider<SessionContext> context={context}>
            {props.children.clone()}
        </ContextProvider<SessionContext>>
    }
}
