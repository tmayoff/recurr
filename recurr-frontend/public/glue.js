const invoke = window.__TAURI__.invoke;

export async function invokeGetSupbaseAuthCredentials() {
  return await invoke("get_supabase_auth_credentials");
}

export async function invokeGetTransactions(
  auth_key,
  access_token,
  start_date,
  end_date,
  options,
) {
  return await invoke("get_transactions", {
    authKey: auth_key,
    accessToken: access_token,
    startDate: start_date,
    endDate: end_date,
    options: options,
  });
}

export async function invokeGetCategories() {
  return await invoke("get_categories");
}

export async function invokeLinkTokenCreate(anon_key, user_id) {
  return await invoke("link_token_create", {
    authKey: anon_key,
    userId: user_id,
  });
}

export async function invokeRemoveAccount(user_id, auth_key, access_token) {
  return await invoke("remove_account", {
    userId: user_id,
    authKey: auth_key,
    accessToken: access_token,
  });
}

export async function invokeItemPublicTokenExchange(anon_key, public_token) {
  return await invoke("item_public_token_exchange", {
    authKey: anon_key,
    publicToken: public_token,
  });
}

export async function invokeSaveAccessToken(auth_token, user_id, access_token) {
  return await invoke("save_access_token", {
    userId: user_id,
    authToken: auth_token,
    accessToken: access_token,
  });
}

export async function invokeSavePlaidAccount(
  auth_token,
  user_id,
  access_token,
  account_id,
) {
  return await invoke("save_plaid_account", {
    authToken: auth_token,
    userId: user_id,
    accessToken: access_token,
    plaidAccountId: account_id,
  });
}

export async function invokeGetPlaidAccounts(auth_token, user_id) {
  return await invoke("get_plaid_accounts", {
    authToken: auth_token,
    userId: user_id,
  });
}

export async function invokeGetPlaidBalances(auth_key, user_id) {
  return await invoke("get_plaid_balances", {
    authKey: auth_key,
    userId: user_id,
  });
}

export function linkStart(link_token, callback) {
  Plaid.create({
    token: link_token,
    onSuccess: (public_token, metadata) => {
      callback({ public_token: public_token, metadata: metadata });
    },
    onLoad: () => {},
    onExit: (err, metadata) => {
      callback({ error: err, metadata: metadata });
    },
    onEvent: (_eventName, _metadata) => {},
  }).open();
}
