# Notice Bot (Trường Đại Học bách khoa - Đại học Đà Nẵng - DUT)

Notice Bot là một ứng dụng viết bằng Rust, dùng để phân tích (parse) thông báo từ HTML và gửi sang Telegram.  
Điểm chính là nội dung thông báo vẫn giữ nguyên liên kết `<a>` để người nhận có thể bấm trực tiếp trong Telegram.

![Preview](https://github.com/Mintori09/notify-bot-dut/tree/main/src/preview.png)

---

## Tính năng

- Phân tích HTML với cấu trúc `div.tbBox` (gồm caption và content).
- Trích xuất dữ liệu:
  - Ngày thông báo (tự nhận diện định dạng `dd/MM/yyyy` và `yyyy-MM-dd`).
  - Tiêu đề.
  - Nội dung chi tiết (giữ nguyên link `<a>`).
- Sinh ra bản ghi `NoticeSent` có `external_id` dựa trên SHA256 của ngày và tiêu đề.
- Gửi thông báo qua Telegram:
  - Sử dụng chế độ `HTML parse mode`.
  - Giữ nguyên link trong nội dung.

---

## Công nghệ

- [Rust](https://www.rust-lang.org/)
- [scraper](https://crates.io/crates/scraper) để phân tích HTML
- [sha2](https://crates.io/crates/sha2) để tạo hash
- [chrono](https://crates.io/crates/chrono) để xử lý ngày giờ
- [teloxide](https://crates.io/crates/teloxide) để gửi tin nhắn Telegram

---

## Cấu trúc chính

- `NoticeSent`: struct chứa dữ liệu thông báo
- `analysis_notice`: phân tích HTML thành danh sách `NoticeSent`
- `send_notice`: gửi thông báo sang Telegram

---

## Cài đặt và chạy

1. Cài Rust:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone dự án:

   ```bash
   git clone https://github.com/Mintori09/notify-bot-dut.git
   cd notice-bot
   ```

3. Thiết lập biến môi trường cho Telegram:

   ```bash
   export DATABASE_URL=your_database_url
   export TELOXIDE_TOKEN=your_bot_token
   export CHAT_ID=your_chat_id
   export FILTER_NOTICE="your_filter","this_is_whitelist"
   ```

4. Chạy chương trình:

   ```bash
   cargo run
   ```

---

## Ví dụ

### HTML đầu vào

```html
<div class="tbBox">
  <div class="tbBoxCaption">
    <b><span>29/08/2025:</span></b>
    <span>Danh sách thi TOEIC ngày 30/08 </span>
  </div>
  <div class="tbBoxContent">
    <p>
      - Sinh viên xem danh sách:
      <a href="https://1drv.ms/b/...">Tại đây</a>
    </p>
  </div>
</div>
```

### Kết quả sau khi gửi Telegram

```
#Exam
Danh sách thi TOEIC ngày 30/08
Date: 2025-08-29
Details:
- Sinh viên xem danh sách: Tại đây
Sent at: 2025-09-01 10:20:30
```

Trong đó chữ **"Tại đây"** là link bấm được.

---

## Lưu ý

- Tin nhắn gửi đi dùng `parse_mode = "HTML"`, vì vậy cần đảm bảo nội dung chỉ chứa các thẻ an toàn (ở đây chỉ giữ `<a>`).
- Các thẻ khác như `<p>`, `<span>`, `<b>` nên loại bỏ để tránh lỗi khi render trên Telegram.
- Giữa lại thẻ `<a>`
