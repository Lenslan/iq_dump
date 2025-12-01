
本仓库代码来自于 
- https://github.com/Lenslan/gain_tester  （Client相关代码）
- https://github.com/1084619673/iq_analyser  （FFT 计算相关代码）

Server 端代码为：
- https://github.com/Lenslan/dumpiq_server

## how to use
直接：
```
python python/main.py
```

实际使用中如果需要有什么改动，可以直接改python 脚本，而不需要像gain_tester 仓库中那样动底层rust代码来重新编译

后续Action：
- [ ] 搞下仪器的api来在脚本中控制仪器
