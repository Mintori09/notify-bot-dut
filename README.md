# Notify Bot DUT

A **Telegram bot** that fetches student/class/fee notices from **Đại học Bách Khoa - ĐHĐN (DUT)** website and automatically sends them to Telegram.

---

## Features

- Fetches notices from DUT student portal:
  - **Training (Đào tạo)**
  - **Class notices (Lớp học phần)**
  - **Student Affairs (Công tác SV)**
  - **Tuition & fees (Học phí, lệ phí)**

- Filters notices (e.g., by class code `23.Nh99`).
- Stores sent notices in SQLite to avoid duplicates.
- Sends formatted messages to Telegram with emojis and HTML styling.

---

## Requirements

- Rust (edition 2021+ recommended)
- SQLite (local file)
- Telegram bot token (from [@BotFather](https://t.me/botfather))
- `chat_id` (user/group/channel)

---

## Setup

Clone the repo:

```bash
git clone https://github.com/yourname/notify-bot-dut.git
cd notify-bot-dut
```

Install dependencies:

```bash
cargo build
```

---

## Environment variables

Set these before running:

```bash
export TELOXIDE_TOKEN="123456:ABC-DEF..."
export CHAT_ID="123456789"   # or negative number for groups/channels
```

Optional:

```bash
export REMINDEE_DB="/path/to/remindee_db.sqlite"
export SQLITE_MAX_CONNECTIONS=1
```

---

## Run the bot

```bash
cargo run --release
```

The bot will:

1. Connect to SQLite.
2. Fetch notices every hour.
3. Insert unseen notices into DB.
4. Send them to Telegram.

---

## Run tests

Unit tests and integration tests:

```bash
cargo test -- --nocapture
```

Special test for Telegram sending:

```bash
cargo test --test send_notice_test -- --nocapture
```

Make sure you exported `TELOXIDE_TOKEN` and `CHAT_ID`.

---

## Example output on Telegram

```
Exam schedule update
Date: 2025-08-31
Details:
Class 23.Nh67 moved to H303
Sent at: 2025-08-31 12:00:00
```

