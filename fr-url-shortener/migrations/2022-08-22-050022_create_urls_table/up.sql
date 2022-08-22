-- Your SQL goes here
CREATE TABLE urls (
    id SERIAL  PRIMARY KEY,
    key VARCHAR(8) UNIQUE NOT NULL,
    secret_key VARCHAR(32) UNIQUE NOT NULL,
    target_url VARCHAR(128) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT 't',
    clicks INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NUll DEFAULT NOW(),
    updated_at TIMESTAMP NOT NUll DEFAULT NOW()
);

CREATE INDEX index_urls_key ON urls(key);
CREATE INDEX index_urls_secret_key ON urls(secret_key);
CREATE INDEX index_urls_target_url ON urls(target_url);