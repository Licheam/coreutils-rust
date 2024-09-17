记得修改脚本里的sed命令

```
sed -i "s/^[[:blank:]]*PATH=.*/  PATH='\/coreutils-rust\/target\/debug\$(PATH_SEPARATOR)'\"\$\$PATH\" \\\/" Makefile
```

为rust转译后的二进制的实际路径