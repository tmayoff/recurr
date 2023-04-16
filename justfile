dev: 
    cargo tauri dev

build:
    cargo tauri build

supabase:
    supabase start && \
    supabase functions serve plaid --env-file supabase/.env
