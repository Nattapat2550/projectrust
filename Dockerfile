# Stage 1: Builder
FROM rust:slim-bookworm AS builder
WORKDIR /app

# ติดตั้ง System dependencies ที่จำเป็นสำหรับการ Compile (เช่น OpenSSL)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy โค้ดทั้งหมดเข้าไปใน Container
COPY . .

# Build โค้ดในโหมด Release (รีดประสิทธิภาพตาม profile.release ใน Cargo.toml)
RUN cargo build --release

# Stage 2: Runtime (สร้าง Image ขนาดเล็กสำหรับรันบน Cloud Run)
FROM debian:bookworm-slim
WORKDIR /app

# ติดตั้ง dependencies ที่จำเป็นตอนรันโปรแกรม
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy ไฟล์ Binary ที่ Build เสร็จแล้วมาจาก Stage 1
COPY --from=builder /app/target/release/pure-api /usr/local/bin/pure-api

# [สำคัญ] Copy โฟลเดอร์ app/ ที่มีไฟล์ .exe และ .apk มาด้วย (ตามที่กำหนดใน env.rs)
COPY --from=builder /app/app ./app

# Expose Port (Cloud Run จะเป็นคนส่ง Environment Variable PORT มาให้เอง)
EXPOSE 8080

# คำสั่งสำหรับเริ่มรัน API
CMD ["pure-api"]