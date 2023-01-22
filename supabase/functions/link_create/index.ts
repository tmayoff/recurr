// Follow this setup guide to integrate the Deno language server with your editor:
// https://deno.land/manual/getting_started/setup_your_environment
// This enables autocomplete, go to definition, etc.

import { serve } from "https://deno.land/std@0.168.0/http/server.ts"

const BASE_URL = "https://sandbox.plaid.com";

serve(async (req: Request) => {
  console.log("Link Create");
  console.log(req);
  
  const plaid_client_id = Deno.env.get('PLAID_CLIENT_ID')!;
  const plaid_secret = Deno.env.get('PLAID_SECRET')!;

  const res = await fetch(BASE_URL + "/link/token/create", {
    method: "POST",
    headers: {
      "content-type": "application/json",
      "PLAID-CLIENT-ID": plaid_client_id,
      'PLAID-SECRET': plaid_secret,
    },
    body: req.body
  });

  return res;
})
