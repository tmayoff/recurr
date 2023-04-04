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


alter table "public"."transactions" enable row level security;

CREATE UNIQUE INDEX plaid_accounts_account_id_key ON public.plaid_accounts USING btree (account_id);

CREATE UNIQUE INDEX transactions_pkey ON public.transactions USING btree (transaction_id, account_id);

alter table "public"."transactions" add constraint "transactions_pkey" PRIMARY KEY using index "transactions_pkey";

alter table "public"."plaid_accounts" add constraint "plaid_accounts_account_id_key" UNIQUE using index "plaid_accounts_account_id_key";

alter table "public"."transactions" add constraint "transactions_account_id_fkey" FOREIGN KEY (account_id) REFERENCES plaid_accounts(account_id) not valid;

alter table "public"."transactions" validate constraint "transactions_account_id_fkey";


