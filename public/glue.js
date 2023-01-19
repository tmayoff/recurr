const invoke = window.__TAURI__.invoke;

export async function invokeLinkCreate() {
    return await invoke("link_create");
}

export async function invokeTokenExchange(public_token) {
    return await invoke("token_exchange", { public_token: public_token });
}

export async function link_start(link_token) {
    Plaid.create({
        token: link_token, onSuccess: (public_token, _metadata) => {
            return public_token;
        },
        onLoad: () => { },
        onExit: (err, _metadata) => {
            console.log(err);
        },
        onEvent: (_eventName, _metadata) => { },
    }).open();
}
