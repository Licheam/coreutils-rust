# Rust转译coreutils

## 使用Docker

1. 使用Docker运行需要先拉取**完整**仓库
```bash
git clone --recurse-submodules https://github.com/licheam/coreutils-rust
```

2. 运行docker
```bash
docker buildx build -t coreutils-rust --platform linux/amd64 -f Dockerfile ./
```

3. coreutils二进制文件目录在`/home/user/coreutils-rust/target/debug`

GNU coreutils二进制测试结果在`/home/user/test/coreutils-origin/tests/test-suite.log`
```
# TOTAL: 631
# PASS:  544
# SKIP:  86
# XFAIL: 0
# FAIL:  1
# XPASS: 0
# ERROR: 0
```

Rust coreutils二进制测试结果在`/home/user/test/coreutils-rust/tests/test-suite.log`
```
# TOTAL: 631
# PASS:  538
# SKIP:  86
# XFAIL: 0
# FAIL:  7
# XPASS: 0
# ERROR: 0
```

新增的错误有：
```
FAIL: tests/misc/invalid-opt
指令: yes
FAIL: tests/misc/join
指令: join
FAIL: tests/misc/cut
指令: cut
FAIL: tests/misc/numfmt
指令: numfmt
FAIL: tests/misc/sort
FAIL: tests/misc/sort-month
指令: sort
```
## 直接编译

1. 拉取仓库
```bash
git clone https://github.com/licheam/coreutils-rust
```

2. 安装coreutils依赖
``` bash
sudo dnf group install -y "Development Tools"
sudo dnf install -y openssl-devel gmp-devel llvm-libs-12.0.1-2.oe2203 llvm-devel-12.0.1-2.oe2203 clang-devel cmake
```

3. 安装Rust

4. 编译coreutils-rust
```bash
cargo build --bins --keep-going -Z unstable-options -Z sparse-registry # 出现几个binary编译失败属正常现象
```

5. 检查二进制文件数量（应当为81）
```bash
find ./target/debug -maxdepth 1 -type f -executable | wc -l
```

## 依赖图

[coreutils依赖关系](dependencies.pdf)
