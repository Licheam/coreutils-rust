FROM openeuler/openeuler:22.03-lts

# 换dnf源
RUN sed -e 's|^metalink=|#metalink=|g' \
    -e 's|^baseurl=http://repo.openeuler.org|baseurl=https://mirrors.tuna.tsinghua.edu.cn/openeuler|g' \
    -e 's|^gpgkey=http://repo.openeuler.org|gpgkey=https://mirrors.tuna.tsinghua.edu.cn/openeuler|g' \
    -i.bak \
    /etc/yum.repos.d/openEuler.repo

RUN dnf update -y
RUN dnf install -y sudo
RUN echo "user ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# 创建用户
RUN useradd -rmg root user
USER user

# 安装coreutils依赖
RUN sudo dnf group install -y "Development Tools"
RUN sudo dnf install -y openssl-devel gmp-devel

RUN sudo dnf install -y llvm-libs-12.0.1-2.oe2203 llvm-devel-12.0.1-2.oe2203 clang-devel cmake curl

# 安装配置rust
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
RUN curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh -s -- --default-toolchain nightly-2022-08-08-x86_64-unknown-linux-gnu -y
ENV PATH=/home/user/.cargo/bin:$PATH
COPY --chown=user .cargo/config.toml /home/user/.cargo/config.toml

# 编译
COPY --chown=user . /home/user/coreutils-rust
WORKDIR /home/user/coreutils-rust
RUN cargo build --bins --keep-going -Z unstable-options -Z sparse-registry || true

RUN find ./target/debug -maxdepth 1 -type f -executable