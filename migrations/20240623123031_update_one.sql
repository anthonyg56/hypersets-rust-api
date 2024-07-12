-- Add migration script here
-- Add a new value to enum type
ALTER TYPE hardware_type
ADD VALUE 'Misc';
-- Update table columns to be more in sync with the rest of the tables
ALTER TABLE presets
  RENAME COLUMN youtube_id TO youtube_url;
ALTER TABLE presets
  RENAME COLUMN name TO preset_name;
-- Update table columns to be more in sync with the rest of the tables
ALTER TABLE games
  RENAME COLUMN created_at TO created_on;
-- Update table columns to be more in sync with the rest of the tables
ALTER TABLE comments
  RENAME COLUMN created_at TO created_on;
-- Insert some values
INSERT INTO presets(
    preset_name,
    download_url,
    description,
    youtube_url,
    photo_url,
    hardware
  )
VALUES (
    'Vaporwave',
    'https://drive.google.com/file/d/19q89S5OR1L0AiLjPLw-gO0LW_JpWvJdM/view?usp=drive_link',
    'The keyboard in the photo is an Alloys origin Core with marshmallow keycaps.',
    null,
    'https://mxmzlgtpvuwhhpsjmxip.supabase.co/storage/v1/object/public/preset%20backgrounds/c7121009-3314-48ac-b48c-6fbccb98099d/F_MYmTOXYAAEo3X.jpg',
    'Keyboard'
  )