dev: 
    cargo tauri dev

build:
    cargo tauri build

functions:
    supabase functions serve plaid --env-file supabase/.env
