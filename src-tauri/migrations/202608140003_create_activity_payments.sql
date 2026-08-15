CREATE TABLE IF NOT EXISTS activity_payments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  activity_id INTEGER NOT NULL,
  student_id INTEGER NOT NULL,
  amount_paid REAL NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'pending'
    CHECK (status IN ('pending', 'partial', 'paid')),
  paid_at DATETIME,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  FOREIGN KEY(activity_id) REFERENCES activities(id) ON DELETE CASCADE,
  FOREIGN KEY(student_id) REFERENCES students(id) ON DELETE CASCADE,

  UNIQUE(activity_id, student_id)
)
