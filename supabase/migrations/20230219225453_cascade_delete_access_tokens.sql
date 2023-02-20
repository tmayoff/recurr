alter table "public"."plaid_accounts" drop constraint "plaid_accounts_access_token_id_fkey";

alter table "public"."plaid_accounts" add constraint "plaid_accounts_access_token_id_fkey" FOREIGN KEY (access_token_id) REFERENCES access_tokens(id) ON DELETE CASCADE not valid;

alter table "public"."plaid_accounts" validate constraint "plaid_accounts_access_token_id_fkey";


