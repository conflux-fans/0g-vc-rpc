# 0g-vc-rpc-service

0g-vc-rpc-service is a JSON-RPC service for [0g-vc](https://github.com/0glabs/0g-vc/)

## Setup

1. Clone 本项目到本地
2. Clone `0g-vc` 到同级目录
3. 参照 0g-vc 中的说明, 编译生成 zk 电路代码(output 文件夹)
4. 将 zk 电路代码(output 文件夹)复制到本项目目录下
4. `cargo run --release` 启动项目, 代码默认监听 `3030` 端口

## API

该 RPC 服务提供两个方法:

- zg_generateZkProof
- zg_verifyZkProof

具体方法的参数和返回值请参看 `src/rpc/api.rs::ZgVc` 定义
