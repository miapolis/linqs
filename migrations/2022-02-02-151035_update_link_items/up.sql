ALTER TABLE link_items
ADD COLUMN user_id INTEGER,
ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id);
