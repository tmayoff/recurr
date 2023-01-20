const invoke = window.__TAURI__.invoke;

export async function invokeLinkCreate() {
    return await invoke("link_create");
}

export async function invokeTokenExchange(public_token) {
    console.log({ publicToken: public_token });
    return await invoke("token_exchange", { publicToken: public_token });
}

export async function linkStart(link_token, callback) {
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
