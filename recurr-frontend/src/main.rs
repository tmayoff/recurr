mod app;
mod auth;
mod commands;
mod context;
mod dashboard;
mod plaid;
mod supabase;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
