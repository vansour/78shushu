# 使用rust:trixie作为基础镜像进行多阶段构建
FROM rust:trixie as builder

# 设置工作目录
WORKDIR /app

# 复制Cargo.toml和Cargo.lock文件以利用Docker缓存
COPY Cargo.toml Cargo.lock ./

# 创建一个虚拟的main.rs来预编译依赖项
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 编译依赖项（这一层会被缓存）
RUN cargo build --release
RUN rm src/main.rs

# 复制源代码
COPY src ./src
COPY static ./static

# 构建应用
RUN touch src/main.rs && cargo build --release

# 运行时镜像 - 使用debian:trixie-slim以保持轻量
FROM debian:trixie-slim as runtime

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -m -u 1000 appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /app/target/release/shushu78 /app/shushu78

# 复制静态文件
COPY --from=builder /app/static /app/static

# 更改文件所有者
RUN chown -R appuser:appuser /app

# 切换到非root用户
USER appuser

# 暴露端口
EXPOSE 3000

# 设置环境变量
ENV RUST_LOG=info

# 启动应用
CMD ["./shushu78"]