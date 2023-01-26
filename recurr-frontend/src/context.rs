use crate::supabase;
use std::rc::Rc;
use supabase_js_rs::SupabaseClient;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub anon_key: String,
    pub supabase_client: SupabaseClient,
    pub supabase_session: Option<supabase::Session>,
}

impl Reducible for Session {
    type Action = Option<supabase::Session>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Session {
            supabase_client: self.supabase_client.clone(),
            supabase_session: action,
            anon_key: self.anon_key.clone(),
        }
        .into()
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
    let url = "https://linaejyblplchxcrusjy.supabase.co";

    let context = use_reducer(|| {
        let anon_key = String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxpbmFlanlibHBsY2h4Y3J1c2p5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzQyNjc3ODMsImV4cCI6MTk4OTg0Mzc4M30.CSc7E2blxAaO2ijXxOGjmhdgmlDVKmBAUSROuWPujWI");
        let session = Session {
            supabase_client: supabase_js_rs::create_client(url, anon_key.as_str()),
            supabase_session: None,
            anon_key,
        };

        session
    });

    html! {
        <ContextProvider<SessionContext> context={context}>
            {props.children.clone()}
        </ContextProvider<SessionContext>>
    }
}
