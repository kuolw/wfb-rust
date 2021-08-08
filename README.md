## Web Framework Benchmarks For Rust

使用 `Apache bench` 工具进行压力测试。
```shell
ab -c 200 -n 200000 -k http://127.0.0.1:8080/
```