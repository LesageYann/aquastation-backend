ALTER TABLE core.user ADD COLUMN token character varying;
ALTER TABLE core.user ADD COLUMN token_created_at TIMESTAMPTZ;