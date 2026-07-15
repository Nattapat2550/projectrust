-- =======================================================
--  UUIDv7 Generator Function
-- =======================================================
CREATE OR REPLACE FUNCTION uuid_generate_v7()
RETURNS uuid
AS $$
DECLARE
  v_unix_t bigint;
  v_rand_a bigint;
  v_rand_b bigint;
  v_rand_c bigint;
BEGIN
  v_unix_t := (extract(epoch from clock_timestamp()) * 1000)::bigint;
  v_rand_a := (random() * 4095)::bigint;
  v_rand_b := (random() * 4095)::bigint;
  v_rand_c := (random() * 281474976710655)::bigint;
  
  RETURN (
    lpad(to_hex(v_unix_t), 12, '0') ||
    '7' || lpad(to_hex(v_rand_a), 3, '0') ||
    to_hex(8 + (random() * 3)::int) || lpad(to_hex(v_rand_b), 3, '0') ||
    lpad(to_hex(v_rand_c), 12, '0')
  )::uuid;
END;
$$ LANGUAGE plpgsql VOLATILE;

-- =======================================================
--  DATABASE SCHEMA สำหรับ pure-api (PostgreSQL)
--  ✅ UUID v7 เป็น Primary Key ทุกตาราง
-- =======================================================

-- -------------------------------------------------------
-- 1) USERS
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
  id                   UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
  username             VARCHAR(50) UNIQUE,
  email                VARCHAR(255) UNIQUE NOT NULL,
  tel                  VARCHAR(20) UNIQUE,                             -- เบอร์โทร (UNIQUE)
  first_name           VARCHAR(100),                                   -- ชื่อจริง
  last_name            VARCHAR(100),                                   -- นามสกุลจริง
  password_hash        VARCHAR(255),
  role                 VARCHAR(10) NOT NULL DEFAULT 'user',
  status               VARCHAR(20) NOT NULL DEFAULT 'active',          -- สถานะบัญชี
  profile_picture_url  TEXT DEFAULT 'assets/user.png',
  is_email_verified    BOOLEAN NOT NULL DEFAULT FALSE,
  oauth_provider       VARCHAR(20),
  oauth_id             VARCHAR(255),
  last_login_at        TIMESTAMPTZ,                                    -- เวลาล็อกอินล่าสุด
  created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT chk_role CHECK (role IN ('user','admin')),
  CONSTRAINT chk_status CHECK (status IN ('active', 'suspended', 'banned', 'deleted')) -- เพิ่ม deleted
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_oauth ON users(oauth_provider, oauth_id);


-- -------------------------------------------------------
-- 2) VERIFICATION CODES (ยืนยันอีเมล)
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS verification_codes (
  id          UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
  user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  code        VARCHAR(6) NOT NULL,
  expires_at  TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_verif_user_exp ON verification_codes(user_id, expires_at);

-- -------------------------------------------------------
-- 3) PASSWORD RESET TOKENS (ลืมรหัสผ่าน)
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS password_reset_tokens (
  id         UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
  user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token      VARCHAR(255) UNIQUE NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  is_used    BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reset_user_exp ON password_reset_tokens(user_id, is_used, expires_at);

-- -------------------------------------------------------
-- 4) HOMEPAGE CONTENT
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS homepage_content (
  section_name VARCHAR(50) PRIMARY KEY,
  content      TEXT NOT NULL,
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- -------------------------------------------------------
-- 5) CAROUSEL ITEMS
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS carousel_items (
  id           UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
  item_index   INTEGER NOT NULL DEFAULT 0,
  title        VARCHAR(255),
  subtitle     VARCHAR(255),
  description  TEXT,
  image_dataurl TEXT NOT NULL,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_carousel_item_index ON carousel_items(item_index, id);

-- -------------------------------------------------------
-- 6) API CLIENTS
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS api_clients (
  id         UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
  name       VARCHAR(100) NOT NULL,
  api_key    VARCHAR(255) NOT NULL UNIQUE,
  is_active  BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_api_clients_active ON api_clients(is_active);

-- -------------------------------------------------------
-- (OPTIONAL) SEED ข้อมูลเริ่มต้น
-- -------------------------------------------------------

-- ตัวอย่าง seed homepage (ลบ/แก้ตามใจได้)
INSERT INTO homepage_content (section_name, content)
VALUES
  ('hero', 'Welcome to MyApp'),
  ('about', 'This is a shared platform for all clients.')
ON CONFLICT (section_name) DO NOTHING;


-- ตัวอย่าง seed api_clients (เปลี่ยน key ให้ตรงกับ .env / config ฝั่ง client)
INSERT INTO api_clients (name, api_key)
VALUES
  ('react-web',      'react-key-123'),
  ('angular-web',    'angular-key-123'),
  ('android-app',    'android-key-123'),
  ('windows-app',    'windows-key-123'),
  ('docker-worker',  'docker-key-123'),
  ('golang-web',     'golang-key-123'),
  ('concert-test',   'concert-key-123'),
  ('mall-test',      'mall-key-123'),
  ('teach-test',     'teach-key-123')
ON CONFLICT (api_key) DO NOTHING;


-- (OPTIONAL) ตัวอย่างสร้าง admin user เปล่า ๆ (ยังไม่มี password)
-- ให้ไปตั้งรหัสผ่านผ่านระบบ หรือ update ทีหลัง
-- จะรันหรือไม่รันก็ได้
/*
INSERT INTO users (username, email, role, is_email_verified)
VALUES ('admin', 'admin@example.com', 'admin', TRUE)
ON CONFLICT (email) DO NOTHING;
*/

-- =======================================================
-- จบสคริปต์
-- =======================================================
