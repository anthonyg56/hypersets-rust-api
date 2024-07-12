-- Add migration script here
ALTER TABLE presets
ALTER COLUMN downloads TYPE integer USING '0'::integer;
ALTER TABLE presets
ALTER COLUMN downloads
SET NOT NULL;