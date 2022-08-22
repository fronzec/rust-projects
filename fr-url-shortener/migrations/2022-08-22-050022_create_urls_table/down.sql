-- This file should undo anything in `up.sql`
DROP INDEX index_urls_key;
DROP INDEX index_urls_secret_key;
DROP INDEX index_urls_target_url;

DROP TABLE urls;