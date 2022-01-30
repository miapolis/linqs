CREATE TABLE link_uses (
    id SERIAL PRIMARY KEY,
    link_item_id VARCHAR(10),
    ip INET,
    user_agent TEXT,
    CONSTRAINT fk_link_item FOREIGN KEY(link_item_id) REFERENCES link_items(id)
);