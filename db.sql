-- =======================================================
--  DATABASE SCHEMA สำหรับ pure-api (PostgreSQL)
-- =======================================================
-- =======================================================
--  DATABASE SCHEMA สำหรับ pure-api (PostgreSQL)
-- =======================================================

-- (ใช้ extension นี้ได้ถ้าต้องการ uuid ฯลฯ)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- -------------------------------------------------------
-- 1) USERS
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
  id                   SERIAL PRIMARY KEY,
  user_id              UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(), -- สุ่ม UUID v4 อัตโนมัติ
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
CREATE INDEX IF NOT EXISTS idx_users_user_id ON users(user_id);


-- -------------------------------------------------------
-- 2) VERIFICATION CODES (ยืนยันอีเมล)
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS verification_codes (
  id          SERIAL PRIMARY KEY,
  user_id     INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  code        VARCHAR(6) NOT NULL,
  expires_at  TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_verif_user_exp ON verification_codes(user_id, expires_at);

-- -------------------------------------------------------
-- 3) PASSWORD RESET TOKENS (ลืมรหัสผ่าน)
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS password_reset_tokens (
  id         SERIAL PRIMARY KEY,
  user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
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
  id           SERIAL PRIMARY KEY,
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
  id         SERIAL PRIMARY KEY,
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
  ('mall-test',      'mall-key-123')
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
