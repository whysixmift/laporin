-- 20260914000000_add_admin_role.sql
-- Add role column to users table and create indexes

ALTER TABLE users ADD COLUMN IF NOT EXISTS role TEXT NOT NULL DEFAULT 'user';
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);

-- Set miftasigma11@gmail.com and default admin as admin
UPDATE users SET role = 'admin' WHERE email = 'miftasigma11@gmail.com';
