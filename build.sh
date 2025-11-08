#!/bin/bash

# Dừng script ngay lập tức nếu có lỗi
set -euo pipefail

BINARY_NAME="notify-bot-dut"

SERVICE_NAME="notify-bot-dut"

BOT_USER=${SUDO_USER:-root}

# 1. Kiểm tra quyền Sudo
if [ "$EUID" -ne 0 ]; then
    echo "Failed: Please run this script with sudo permission."
    exit 1
fi

if [ "$BOT_USER" == "root" ]; then
    echo "-----------------------------------------------------"
    echo "Warn: Don't find SUDO_USER."
    echo "Enter for continue, or Ctrl+C to stop..."
    read -r
    echo "-----------------------------------------------------"
fi

# (Nơi chứa file .env)
PROJECT_DIR=$(pwd)
echo "Thư mục dự án: $PROJECT_DIR"
echo "Dịch vụ sẽ chạy với user: $BOT_USER"

# `systemctl is-active` trả về 0 nếu active, nên chúng ta bỏ qua lỗi (dùng `|| true`)
if systemctl is-active --quiet "$SERVICE_NAME"; then
    echo "Dịch vụ '$SERVICE_NAME' đang chạy. Tạm dừng để cập nhật..."
    sudo systemctl stop "$SERVICE_NAME"
fi

echo "Compiling ..."
sudo -u "$BOT_USER" cargo build --release
echo "Compile completed."

BINARY_PATH="./target/release/$BINARY_NAME"
if [ ! -f "$BINARY_PATH" ]; then
    echo "Failed: Can't find file builded at '$BINARY_PATH'"
    exit 1
fi

echo "Copy '$BINARY_PATH' to '/usr/local/bin/$BINARY_NAME'..."
cp "$BINARY_PATH" "/usr/local/bin/$BINARY_NAME"
chmod +x "/usr/local/bin/$BINARY_NAME"

SERVICE_FILE_PATH="/etc/systemd/system/$SERVICE_NAME.service"
echo "Create: $SERVICE_FILE_PATH"

cat <<EOF >"$SERVICE_FILE_PATH"
[Unit]
Description=Notify Bot DUT ($SERVICE_NAME)
After=network-online.target
Wants=network-online.target

[Service]
User=$BOT_USER
Group=$(id -gn "$BOT_USER")

WorkingDirectory=$PROJECT_DIR

ExecStart=/usr/local/bin/$BINARY_NAME

EnvironmentFile=$PROJECT_DIR/.env

Restart=always
RestartSec=10

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

echo "Restart systemd daemon..."
sudo systemctl daemon-reload

sudo systemctl enable $SERVICE_NAME
sudo systemctl start $SERVICE_NAME

echo "-----------------------------------------------------"
echo "Finish!"

echo "  Status bot:"
echo "  systemctl status $SERVICE_NAME"
echo "-----------------------------------------------------"
