CREATE TABLE IF NOT EXISTS "CnnvdCollect" (
    id BIGSERIAL,
    cnnvd_id text,
    cnnvd_code text,
    cnnvd_source_json text NOT NULL,
    vul_type text,

    CONSTRAINT "CnnvdCollect_pkey" PRIMARY KEY (id),
    CONSTRAINT "CnnvdCollect_unique_key" UNIQUE(cnnvd_id, cnnvd_code,vul_type)
);
CREATE TABLE IF NOT EXISTS "CnnvdProviderUpdates" (
    token text NOT NULL,
    cnnvd_collect_id bigint NOT NULL,
    CONSTRAINT "CnnvdProviderUpdates_pkey" PRIMARY KEY (token, cnnvd_collect_id)
);
CREATE TABLE IF NOT EXISTS "CnnvdProviderToken" (token text PRIMARY KEY);
CREATE TABLE IF NOT EXISTS "CnnvdCollectUpdate" (last_counts bigint PRIMARY KEY);
create index "cnnvd_source_json_bidx" on public."CnnvdCollect"(( cnnvd_source_json=''));