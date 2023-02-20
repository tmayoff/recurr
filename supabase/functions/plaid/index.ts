// Follow this setup guide to integrate the Deno language server with your editor:
// https://deno.land/manual/getting_started/setup_your_environment
// This enables autocomplete, go to definition, etc.

import { serve } from "https://deno.land/std@0.168.0/http/server.ts";

const BASE_URL = "https://sandbox.plaid.com";

serve(async (req: Request) => {
  const plaid_client_id = Deno.env.get("PLAID_CLIENT_ID")!;
  const plaid_secret = Deno.env.get("PLAID_SECRET")!;

  const body = await req.json();
  const endpoint = body["endpoint"];
  const request = body["data"];

  const res = await fetch(BASE_URL + endpoint, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      "PLAID-CLIENT-ID": plaid_client_id,
      "PLAID-SECRET": plaid_secret,
    },
    body: JSON.stringify(request),
  });

  return res;
});
