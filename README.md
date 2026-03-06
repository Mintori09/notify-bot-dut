# notify-bot-dut

Telegram bot viết bằng Rust, tự động theo dõi và gửi thông báo từ trang web Trường Đại học Bách khoa - Đại học Đà Nẵng (DUT).

![Preview](https://github.com/Mintori09/notify-bot-dut/blob/main/images/preview.png)

---

## Tính năng

- Tự động lấy thông báo từ các danh mục: Đào tạo, Thông báo lớp, Công tác sinh viên, Học phí.
- Phân tích HTML với cấu trúc `div.tbBox`, trích xuất tiêu đề, ngày đăng, và nội dung (giữ nguyên link `<a>`).
- Tạo `external_id` bằng SHA-256 của ngày + tiêu đề để tránh gửi trùng.
- Lưu lịch sử thông báo vào SQLite (`~/.config/notify-bot-dut/college.db`) — không cần cài database.
- Gửi tin nhắn qua Telegram với `HTML parse mode`, hỗ trợ link bấm được.
- Tự động tạo bảng khi khởi động lần đầu — không cần chạy migration thủ công.
- Lịch chạy định kỳ, tự chờ khi mất mạng và xử lý rate-limit của Telegram.

---

## Công nghệ

- [Rust](https://www.rust-lang.org/)
- [sqlx](https://crates.io/crates/sqlx) + SQLite — lưu trữ nhẹ, không cần server
- [teloxide](https://crates.io/crates/teloxide) — Telegram bot
- [scraper](https://crates.io/crates/scraper) — phân tích HTML
- [reqwest](https://crates.io/crates/reqwest) — HTTP client
- [chrono](https://crates.io/crates/chrono) — xử lý ngày giờ
- [sha2](https://crates.io/crates/sha2) — tạo hash

---

## Cài đặt và chạy

### 1. Cài Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone và build

```bash
git clone https://github.com/Mintori09/notify-bot-dut.git
cd notify-bot-dut
cargo build --release
```

### 3. Thiết lập biến môi trường

Tạo file `.env` (hoặc export trực tiếp):

```bash
export TELOXIDE_TOKEN=your_bot_token
export CHAT_ID=your_chat_id
# Tuỳ chọn: lọc chỉ giữ thông báo có chứa từ khoá (whitelist, phân cách bằng dấu phẩy)
export FILTER_NOTICE=keyword1,keyword2
```

> **DATABASE_URL không cần thiết.** Database SQLite tự động lưu tại `~/.config/notify-bot-dut/college.db`.

### 4. Chạy

```bash
./target/release/notify-bot-dut
```

---

## Cấu trúc dự án

```
src/
├── main.rs        # Entrypoint, kết nối DB và khởi động scheduler
├── bot.rs         # Vòng lặp fetch → insert → gửi Telegram
├── controller.rs  # Truy vấn DB (sqlx): check_and_insert, mark_as_sent, get_unsent
├── database.rs    # Kết nối SQLitePool, ensure_schema, Config
├── entity.rs      # Struct NoticeSent, enum Category
├── fetch.rs       # Fetch và parse HTML từ website DUT
├── scheduler.rs   # Chạy task định kỳ
└── utils.rs       # Lọc thông báo theo FILTER_NOTICE
```

---

## Ví dụ tin nhắn Telegram

```
#Training
<b>Danh sách thi TOEIC ngày 30/08</b>
<b>Date:</b> 2025-08-29
<b>Details:</b>
- Sinh viên xem danh sách: <a href="...">Tại đây</a>
<i>Sent at: 2025-09-01 10:20:30</i>
```

Chữ **"Tại đây"** là link bấm được trực tiếp trong Telegram.
