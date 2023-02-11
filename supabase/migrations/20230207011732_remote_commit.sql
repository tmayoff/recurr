create table "public"."budgets" (
    "created_at" timestamp with time zone default now(),
    "category_id" text not null,
    "user_id" uuid not null,
    "max" real not null
);


alter table "public"."budgets" enable row level security;

CREATE UNIQUE INDEX budgets_pkey ON public.budgets USING btree (category_id, user_id);

alter table "public"."budgets" add constraint "budgets_pkey" PRIMARY KEY using index "budgets_pkey";

alter table "public"."budgets" add constraint "budgets_user_id_fkey" FOREIGN KEY (user_id) REFERENCES auth.users(id) not valid;

alter table "public"."budgets" validate constraint "budgets_user_id_fkey";

create policy "authenticated users"
on "public"."budgets"
as permissive
for all
to authenticated
using ((auth.uid() = user_id))
with check ((auth.uid() = user_id));



set check_function_bodies = off;

CREATE OR REPLACE FUNCTION storage.extension(name text)
 RETURNS text
 LANGUAGE plpgsql
AS $function$
DECLARE
_parts text[];
_filename text;
BEGIN
    select string_to_array(name, '/') into _parts;
    select _parts[array_length(_parts,1)] into _filename;
    -- @todo return the last part instead of 2
    return split_part(_filename, '.', 2);
END
$function$
;

CREATE OR REPLACE FUNCTION storage.filename(name text)
 RETURNS text
 LANGUAGE plpgsql
AS $function$
DECLARE
_parts text[];
BEGIN
    select string_to_array(name, '/') into _parts;
    return _parts[array_length(_parts,1)];
END
$function$
;

CREATE OR REPLACE FUNCTION storage.foldername(name text)
 RETURNS text[]
 LANGUAGE plpgsql
AS $function$
DECLARE
_parts text[];
BEGIN
    select string_to_array(name, '/') into _parts;
    return _parts[1:array_length(_parts,1)-1];
END
$function$
;


