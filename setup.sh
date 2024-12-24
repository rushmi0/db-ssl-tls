#!/bin/bash

set -e
set -o pipefail

CERTS_DIR="./certs"

echo "===== Initializing Project Setup ====="

# ตรวจสอบว่ามีคำสั่ง Openssl ติดตั้งอยู่หรือไม่
if ! command -v openssl &> /dev/null; then
    echo "Error: openssl is not installed. Please install it and try again."
    exit 1
fi

# สร้างโฟลเดอร์ certs หากยังไม่มี
if [ ! -d "$CERTS_DIR" ]; then
    mkdir "$CERTS_DIR"
    echo "📁 Created directory for certificates: $CERTS_DIR"
else
    echo "📁 Certificates directory exists: $CERTS_DIR"
fi

# ตรวจสอบใบรับรอง SSL/TLS
if [ ! -f "$CERTS_DIR/server.key" ] || [ ! -f "$CERTS_DIR/server.crt" ] || [ ! -f "$CERTS_DIR/root.crt" ]; then
    echo "🔑 Generating SSL Certificates..."

    # สร้าง Private Key
    openssl genrsa -out "$CERTS_DIR/server.key" 2048
    chmod u=rw,g=,o= "$CERTS_DIR/server.key"  # เพิ่มสิทธิ์ให้เฉพาะผู้ใช้เท่านั้น
    echo "✔️  Server private key generated: $CERTS_DIR/server.key"

    # สร้าง Certificate Signing Request (CSR)
    openssl req -new -key "$CERTS_DIR/server.key" -out "$CERTS_DIR/server.csr" \
      -subj "/CN=localhost"
    echo "✔️  CSR generated: $CERTS_DIR/server.csr"

    # ออกใบ Certificate เป็น Self-signed
    openssl x509 -req -days 365 -in "$CERTS_DIR/server.csr" \
      -signkey "$CERTS_DIR/server.key" -out "$CERTS_DIR/server.crt"
    chmod u=rwx,g=r,o=r "$CERTS_DIR/server.crt"  # การตั้งสิทธิ์ให้เฉพาะ `user` และ `group` อ่านได้
    echo "✔️  Self-signed certificate generated: $CERTS_DIR/server.crt"

    # สร้าง Root Certificate
    cp "$CERTS_DIR/server.crt" "$CERTS_DIR/root.crt"
    chmod u=rwx,g=r,o=r "$CERTS_DIR/root.crt"  # ให้สิทธิ์การอ่านกับ user/group/others
    echo "✔️  Root certificate generated: $CERTS_DIR/root.crt"

    # ลบ CSR ที่ไม่จำเป็น
    rm "$CERTS_DIR/server.csr"
    echo "✔️  Temporary CSR removed."
else
    echo "✔️  SSL Certificates already exist, skipping creation step."
fi

# ตรวจสอบว่ามี Docker ติดตั้งอยู่หรือไม่
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed. Please install it and try again."
    exit 1
fi

# ตรวจสอบว่า Docker กำลังทำงานหรือไม่
if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker is not running. Please start it and try again."
    exit 1
fi

# รัน Docker Compose Services
echo "🚀 Starting Docker Compose..."
docker-compose down > /dev/null 2>&1 || true  # หยุด container เก่าหากยังคงทำงานอยู่
docker compose up -d actix-db actix-app
echo "✔️  Docker Compose services are up and running."

echo "===== Setup Completed Successfully! ====="