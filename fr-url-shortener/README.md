# URL Shortener

## Example data for DB

```sql
INSERT INTO public.urls (id, key, secret_key, target_url, is_active, clicks, created_at, updated_at) VALUES (1, 'a', 'aa', 'aaaa', true, 0, '2022-08-25 00:31:22.000000', '2022-08-25 05:31:24.097450');
INSERT INTO public.urls (id, key, secret_key, target_url, is_active, clicks, created_at, updated_at) VALUES (2, 'b', 'bb', 'bbbb', true, 0, '2022-08-25 00:31:22.000000', '2022-08-25 05:31:24.097450');
INSERT INTO public.urls (id, key, secret_key, target_url, is_active, clicks, created_at, updated_at) VALUES (3, 'c', 'cc', 'cccc', true, 0, '2022-08-25 00:31:22.000000', '2022-08-25 05:31:24.097450');
INSERT INTO public.urls (id, key, secret_key, target_url, is_active, clicks, created_at, updated_at) VALUES (4, 'd', 'dd', 'ddd', true, 0, '2022-08-25 00:31:22.000000', '2022-08-25 05:31:24.097450');
INSERT INTO public.urls (id, key, secret_key, target_url, is_active, clicks, created_at, updated_at) VALUES (5, 'e', 'ee', 'eee', true, 0, '2022-08-25 00:31:22.000000', '2022-08-25 05:31:24.097450');
```