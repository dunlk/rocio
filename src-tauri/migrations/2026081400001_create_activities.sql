CREATE TABLE IF NOT EXISTS activities (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  description TEXT,
  activity_type TEXT NOT NULL
    CHECK (activity_type IN ('monthly', 'occacional')),
  amount REAL NOT NULL,
  activities_date DATE,
  due_date DATE,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
