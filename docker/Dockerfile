FROM postgres:14

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.83.0 \
    PGRX_HOME=/var/lib/postgresql/.pgrx

USER postgres
RUN mkdir -p $PGRX_HOME

USER root
RUN apt-get update && apt-get install -y \
    gosu \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libreadline-dev \
    zlib1g-dev \
    libpq-dev \
    bison \
    flex \
    mecab \
    libmecab-dev \
    mecab-ipadic-utf8 \
    # rustのinstall
    && curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $RUST_VERSION

ENV PATH="/usr/local/bin:${PATH}"

# pgrx の依存関係インストール
RUN apt-get install -y \
    postgresql-server-dev-14 \
    clang \
    libclang-dev

# pgrx のインストール
RUN cargo install cargo-pgrx
RUN cargo pgrx init

RUN chmod 777 -R /usr/local/cargo \
    && chown -R postgres:postgres /var/lib/postgresql \
    && chown -R postgres:postgres /usr/lib/postgresql \
    && chown -R postgres:postgres /usr/share/postgresql 