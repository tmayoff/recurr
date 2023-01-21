mod app;
mod auth;
mod context;
mod dashboard;
mod supabase;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
