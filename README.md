# notify-bot-dut

Telegram bot viết bằng Rust, tự động theo dõi và gửi thông báo từ website [Đại học Bách khoa - Đại học Đà Nẵng (DUT)](https://sv.dut.udn.vn).

![Preview](https://github.com/Mintori09/notify-bot-dut/blob/main/images/preview.png)

---

## Tính năng

- Lấy thông báo từ 4 danh mục: **Đào tạo**, **Thông báo lớp**, **Công tác sinh viên**, **Học phí**
- Phân tích HTML, trích xuất tiêu đề, ngày đăng và nội dung (giữ nguyên link `<a>`)
- Tạo `external_id` bằng SHA-256 để tránh gửi trùng
- Lưu lịch sử vào SQLite tại `~/.config/notify-bot-dut/college.db` — không cần cài database
- Gửi tin qua Telegram với `HTML parse mode`, hỗ trợ link bấm được
- Tự tạo bảng khi khởi động lần đầu — không cần migration thủ công
- Tự chờ khi mất mạng, xử lý rate-limit Telegram
- Lọc thông báo lớp theo **whitelist từ khoá** (ví dụ: lớp học phần)

---

## Công nghệ

| Crate | Mục đích |
|-------|----------|
| [teloxide](https://crates.io/crates/teloxide) | Telegram bot |
| [sqlx](https://crates.io/crates/sqlx) + SQLite | Lưu trữ, không cần server |
| [scraper](https://crates.io/crates/scraper) | Phân tích HTML |
| [reqwest](https://crates.io/crates/reqwest) | HTTP client |
| [clap](https://crates.io/crates/clap) | CLI |
| [chrono](https://crates.io/crates/chrono) | Xử lý ngày giờ |
| [serde_json](https://crates.io/crates/serde_json) | Đọc config JSON |

---

## Cài đặt

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

### 3. Thiết lập config

```bash
./target/release/notify-bot-dut config
```

Lệnh trên mở file `~/.config/notify-bot-dut/config.json` trong editor (tự tạo nếu chưa có):

```json
{
  "teloxide_token": "YOUR_BOT_TOKEN",
  "chat_id": -123456789,
  "filter_notice": ["23.Nh16", "23.Nh44"]
}
```

> `filter_notice` là danh sách **whitelist** cho thông báo lớp (`ClassNotice`). Để trống `[]` để nhận tất cả.

### 4. Chạy

```bash
./target/release/notify-bot-dut
```

---

## CLI

```
USAGE:
    notify-bot-dut [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -t, --token <TOKEN>      Override bot token (env: TELOXIDE_TOKEN)
    -c, --chat-id <ID>       Override chat/group ID (env: CHAT_ID)
    -f, --filter <PATTERN>   Override class filter (có thể lặp nhiều lần)

SUBCOMMANDS:
    config           Mở config.json trong $EDITOR
    install-service  Tạo và bật systemd user service
```

**Ví dụ:**

```bash
# Override token và group tạm thời
notify-bot-dut --token 123:ABC --chat-id -987654321

# Override bộ lọc lớp
notify-bot-dut --filter 23.Nh16 --filter 23.Nh44

# Cài service chạy nền tự động khi đăng nhập
notify-bot-dut install-service
```

CLI flag có **độ ưu tiên cao hơn** `config.json`.

---

## Chạy như systemd user service

```bash
# Cài đặt (tự sinh file .service và bật service)
notify-bot-dut install-service

# Kiểm tra trạng thái
systemctl --user status notify-bot-dut

# Xem log
journalctl --user -u notify-bot-dut -f
```

---

## Cấu trúc dự án

```
src/
├── main.rs        # Entrypoint: parse CLI, kết nối DB, khởi động scheduler
├── cli.rs         # Clap CLI: subcommands (config, install-service) và override flags
├── bot.rs         # Vòng lặp fetch → insert → gửi Telegram
├── controller.rs  # Truy vấn DB: check_and_insert, mark_as_sent, get_unsent
├── database.rs    # Config (load từ JSON), kết nối SQLitePool, ensure_schema
├── entity.rs      # Struct NoticeSent, enum Category
├── fetch.rs       # Fetch và parse HTML từ website DUT
├── scheduler.rs   # Chạy task định kỳ
└── utils.rs       # Lọc thông báo theo filter
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
