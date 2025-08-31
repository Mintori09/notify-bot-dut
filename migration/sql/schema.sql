CREATE TABLE IF NOT EXISTS notice_sent (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Only accept 4 categories
    main_category   TEXT NOT NULL 
                    CHECK(main_category IN ('Training', 'ClassNotice', 'StudentAffairs', 'Tuition')),

    -- Unique ID per source (must not be empty)
    external_id     TEXT NOT NULL 
                    CHECK (length(external_id) > 0),

    -- YYYY-MM-DD or NULL
    published_date  TEXT 
                    CHECK (published_date IS NULL OR published_date GLOB '[0-9][0-9][0-9][0-9]-[0-1][0-9]-[0-3][0-9]'),

    -- Body may be NULL, but if present it cannot be just empty string
    body            TEXT CHECK (body IS NULL OR length(trim(body)) > 0),

    -- Title required, at least 3 characters
    title           TEXT NOT NULL CHECK (length(trim(title)) >= 3),

    -- Sent timestamp: default now(), must be valid ISO
    sent_at         TEXT NOT NULL DEFAULT (datetime('now'))
                    CHECK (sent_at GLOB '____-__-__ __:__:__'),

    -- Prevent duplicates in same category
    UNIQUE(main_category, external_id)
);


