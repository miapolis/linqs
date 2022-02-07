CREATE TYPE track_item as ENUM ('time', 'ip', 'user_agent');
ALTER TABLE link_items
ADD COLUMN uses INTEGER DEFAULT 0,
  ADD COLUMN to_track track_item [] DEFAULT ARRAY []::track_item [];
ALTER TABLE link_uses
ALTER COLUMN ip TYPE TEXT,
  ALTER COLUMN user_agent TYPE VARCHAR(500),
  ALTER COLUMN ts TYPE TIMESTAMP;
