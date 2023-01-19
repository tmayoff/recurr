const invoke = window.__TAURI__.invoke;

export async function invokeLinkCreate() {
    return await invoke("link_create");
}

export async function link_start(link_token) {
    console.log("Plaid Initialize");
    Plaid.create({
        token: link_token, onSuccess: (public_token, metadata) => {
            console.log(public_token)
        },
        onLoad: () => { },
        onExit: (err, metadata) => { },
        onEvent: (eventName, metadata) => { },
    }).open();
}
