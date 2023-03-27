CREATE TABLE IF NOT EXISTS users (
  id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  username   TEXT NOT NULL UNIQUE,
  password   TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_permissions (
    user_id  INTEGER NOT NULL,
    token    TEXT NOT NULL
);

-- INSERT INTO users (id, anonymous, username, password) 
-- SELECT 0, true, 'Guest', ''
-- ON CONFLICT(id) DO UPDATE SET
--     anonymous = EXCLUDED.anonymous,
--     username = EXCLUDED.username;


CREATE TABLE IF NOT EXISTS todos (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id    INTEGER NOT NULL,
  title      TEXT NOT NULL,
  completed  BOOLEAN,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  -- FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS stations (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  name       TEXT NOT NULL, 
  address    TEXT NOT NULL,
  city       TEXT NOT NULL,
  phone      TEXT NOT NULL,
  hyperlink  TEXT NOT NULL,
  amount_a_plus INTEGER NOT NULL,
  amount_a_minus INTEGER NOT NULL,
  amount_b_plus INTEGER NOT NULL,
  amount_b_minus INTEGER NOT NULL,
  amount_ab_plus INTEGER NOT NULL,
  amount_ab_minus INTEGER NOT NULL,
  amount_o_plus INTEGER NOT NULL,
  amount_o_minus INTEGER NOT NULL,
);


INSERT INTO stations (name, address, city, phone, hyperlink, amount_a_plus, amount_a_minus, amount_b_plus, amount_b_minus, amount_ab_plus, amount_ab_minus, amount_o_plus, amount_o_minus) VALUES ('Blood Bank', 'Kathmandu', 'Kathmandu', '01-4444444', 'https://www.google.com', 10, 10, 10, 10, 10, 10, 10, 10);
