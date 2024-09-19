#!/bin/bash

# 创建一个文件夹专门用于功能测试
mkdir -p FunctionTest
cd FunctionTest


# 安装测试依赖
sudo dnf install -y wget openssl-devel rsync gperf texinfo

# 下载coreutils源码
git clone --recurse-submodules https://github.com/coreutils/coreutils.git
cd coreutils
git fetch --all --tags 
git checkout tags/v9.0

cd ../

cp -r coreutils rust-coreutils

cd coreutils
export FORCE_UNSAFE_CONFIGURE=1 
./bootstrap --skip-po --force 
./configure --with-openssl --enable-install-program=arch --enable-no-install-program=kill,uptime
make check -j RUN_EXPENSIVE_TESTS=yes RUN_VERY_EXPENSIVE_TESTS=yes
# find result in ./tests/test-suite.log

cd ../rust-coreutils
export FORCE_UNSAFE_CONFIGURE=1
./bootstrap --skip-po --force 
./configure --with-openssl --enable-install-program=arch --enable-no-install-program=kill,uptime
sed -i "s/^[[:blank:]]*PATH=.*/  PATH='\/home\/user\/coreutils-rust\/target\/debug\$(PATH_SEPARATOR)'\"\$\$PATH\" \\\/" Makefile
make check -j RUN_EXPENSIVE_TESTS=yes RUN_VERY_EXPENSIVE_TESTS=yes
