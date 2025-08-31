CREATE TABLE IF NOT EXISTS notice_sent (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    main_category  TEXT NOT NULL,       -- DaoTao | LopHocPhan | CTSV | HocPhi
    external_id    TEXT NOT NULL,       -- hash(title+date) hoặc id nguồn
    published_date TEXT,                -- ngày gốc YYYY-MM-DD (nếu có)
    title          TEXT NOT NULL,       -- tiêu đề chính
    sent_at        TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(main_category, external_id)
);


