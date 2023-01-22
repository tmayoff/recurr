const invoke = window.__TAURI__.invoke;

export async function invokeLinkTokenCreate(anon_key) {
    return await invoke("link_token_create", {anonKey: anon_key});
}

export async function invokeItemPublicTokenExchange(anon_key, public_token) {
    return await invoke("item_public_token_exchange", { anonKey: anon_key, publicToken: public_token });
}

export function linkStart(link_token, callback) {
    Plaid.create({
        token: link_token, onSuccess: (public_token, _metadata) => {
            callback({ public_token: public_token });
        },
        onLoad: () => { },
        onExit: (err, metadata) => {
            callback({ error: err, metadata: metadata });
        },
        onEvent: (_eventName, _metadata) => { },
    }).open();
}
