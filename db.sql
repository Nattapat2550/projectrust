-- =======================================================
--  DATABASE SCHEMA สำหรับ pure-api (PostgreSQL)
-- =======================================================

-- (ใช้ extension นี้ได้ถ้าต้องการ uuid ฯลฯ แต่ตอนนี้ยังไม่จำเป็น)
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- -------------------------------------------------------
-- 1) USERS
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
  id                   SERIAL PRIMARY KEY,
  username             VARCHAR(50) UNIQUE,
  email                VARCHAR(255) UNIQUE NOT NULL,
  password_hash        VARCHAR(255),
  role                 VARCHAR(10) NOT NULL DEFAULT 'user',
  profile_picture_url  TEXT DEFAULT 'assets/user.png',
  is_email_verified    BOOLEAN NOT NULL DEFAULT FALSE,
  oauth_provider       VARCHAR(20),
  oauth_id             VARCHAR(255),
  created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT chk_role CHECK (role IN ('user','admin'))
);

CREATE INDEX IF NOT EXISTS idx_users_email
  ON users(email);

CREATE INDEX IF NOT EXISTS idx_users_oauth
  ON users(oauth_provider, oauth_id);


-- -------------------------------------------------------
-- 2) VERIFICATION CODES (ยืนยันอีเมล)
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS verification_codes (
  id          SERIAL PRIMARY KEY,
  user_id     INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  code        VARCHAR(6) NOT NULL,
  expires_at  TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_verif_user_exp
  ON verification_codes(user_id, expires_at);


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

CREATE INDEX IF NOT EXISTS idx_reset_user_exp
  ON password_reset_tokens(user_id, is_used, expires_at);


-- -------------------------------------------------------
-- 4) HOMEPAGE CONTENT (section ต่าง ๆ หน้าแรก)
--    ใช้กับ /api/homepage
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS homepage_content (
  section_name VARCHAR(50) PRIMARY KEY,
  content      TEXT NOT NULL,
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- -------------------------------------------------------
-- 5) CAROUSEL ITEMS (สไลด์บนหน้า home)
--    ใช้กับ /api/carousel และ /api/admin/carousel
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS carousel_items (
  id           SERIAL PRIMARY KEY,
  item_index   INTEGER NOT NULL DEFAULT 0,
  title        VARCHAR(255),
  subtitle     VARCHAR(255),
  description  TEXT,
  image_dataurl TEXT NOT NULL, -- เก็บเป็น data URL (base64)
  created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_carousel_item_index
  ON carousel_items(item_index, id);


-- -------------------------------------------------------
-- 6) API CLIENTS (แยก API key ของแต่ละ client)
--    ใช้ร่วมกับ middleware apiKeyAuth.ts
-- -------------------------------------------------------
CREATE TABLE IF NOT EXISTS api_clients (
  id         SERIAL PRIMARY KEY,
  name       VARCHAR(100) NOT NULL,   -- เช่น 'react-web', 'android-app'
  api_key    VARCHAR(255) NOT NULL UNIQUE,
  is_active  BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_api_clients_active
  ON api_clients(is_active);


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
  ('docker-worker',  'docker-key-123')
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
