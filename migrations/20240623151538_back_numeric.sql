-- Add migration script here
ALTER TABLE presets
ALTER COLUMN downloads TYPE numeric USING '0'::numeric;
ALTER TABLE presets
ALTER COLUMN downloads
SET NOT NULL;