create table "public"."transactions" (
    "transaction_id" text not null,
    "amount" double precision,
    "category" text,
    "category_id" text,
    "date" date,
    "merchant_name" text,
    "pending" boolean,
    "pending_trasaction_id" text,
    "account_id" text not null
);


CREATE UNIQUE INDEX plaid_accounts_account_id_key ON public.plaid_accounts USING btree (account_id);

CREATE UNIQUE INDEX transactions_pkey ON public.transactions USING btree (transaction_id, account_id);

alter table "public"."transactions" add constraint "transactions_pkey" PRIMARY KEY using index "transactions_pkey";

alter table "public"."plaid_accounts" add constraint "plaid_accounts_account_id_key" UNIQUE using index "plaid_accounts_account_id_key";

alter table "public"."transactions" add constraint "transactions_account_id_fkey" FOREIGN KEY (account_id) REFERENCES plaid_accounts(account_id) not valid;

alter table "public"."transactions" validate constraint "transactions_account_id_fkey";

create policy "Authenticated Users Only"
on "public"."transactions"
as permissive
for all
to authenticated
using ((auth.uid() IN ( SELECT plaid_accounts.user_id
   FROM plaid_accounts
  WHERE (plaid_accounts.account_id = plaid_accounts.account_id))))
with check ((auth.uid() IN ( SELECT plaid_accounts.user_id
   FROM plaid_accounts
  WHERE (plaid_accounts.account_id = plaid_accounts.account_id))));



