# ================================
# 阶段1: 安装 cargo-chef
# ================================
FROM rust:1.92.0-slim-bullseye AS chef

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    build-essential \
    cmake \
    clang \
    perl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked
WORKDIR /app

# ================================
# 阶段2: 分析依赖，生成 recipe.json
# ================================
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY vendor/ vendor/

# 生成依赖配方（只包含依赖信息，不包含源代码）
RUN cargo chef prepare --recipe-path recipe.json

# ================================
# 阶段3: 缓存依赖编译
# ================================
FROM chef AS cacher

COPY --from=planner /app/recipe.json recipe.json
COPY vendor/ vendor/

# 只编译依赖，这一层会被缓存
# 只有 Cargo.toml/Cargo.lock 变化时才重新执行
RUN cargo chef cook --release --recipe-path recipe.json

# ================================
# 阶段4: 构建业务代码
# ================================
FROM chef AS builder

# 复制已编译的依赖
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# 复制源代码
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY vendor/ vendor/

# 只编译业务代码（依赖已缓存）
RUN cargo build --release -p web-server

# ================================
# 阶段5: 运行时镜像
# ================================
FROM debian:bullseye-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/web-server /app/web-server

# Bundle SQL migrations for multi-server deployment (no volume mount needed)
COPY sql/ /app/sql/

# 创建非 root 用户
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["/app/web-server"]
