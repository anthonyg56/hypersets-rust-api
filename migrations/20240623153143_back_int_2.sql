-- Add migration script here
ALTER TABLE presets
ALTER COLUMN views TYPE integer USING '0'::integer;
ALTER TABLE presets
ALTER COLUMN views
SET NOT NULL;