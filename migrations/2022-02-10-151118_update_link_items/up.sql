ALTER TABLE link_items
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  ADD COLUMN expires_at TIMESTAMP DEFAULT NULL,
  ADD COLUMN max_uses INTEGER DEFAULT NULL;