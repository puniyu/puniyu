# puniyu_server

puniyu Web服务器模块

## 概述

`puniyu_server` 是 puniyu 项目中的 Web 服务器核心库，基于 `actix-web` 框架构建。它提供了 HTTP 服务器的启动、日志记录、访问日志中间件等核心功能。

## 核心功能

### 服务器启动

- `run_server`: 启动 HTTP 服务器的主函数
- `run_server_spawn`: 在后台线程中启动 HTTP 服务器

服务器配置：

- 默认监听地址：`127.0.0.1:33720`
- 可通过环境变量 `HTTP_HOST` 和 `HTTP_PORT` 自定义
- 支持通过参数指定监听地址和端口

### 日志系统

- `log_init`: 初始化日志系统
- 支持环境变量配置日志级别
- 提供统一的日志宏：`info!`、`warn!`、`error!`、`debug!`

日志配置：

- 默认日志级别：`info`
- 可通过 `LOGGER_LEVEL` 环境变量设置
- 日志前缀统一为 `Server`

### 中间件

#### AccessLog 中间件

记录 HTTP 请求访问日志：

- 记录请求方法、路径、响应状态码
- 记录请求处理耗时（毫秒）
- 记录客户端 IP 地址
- 支持从 HTTP 头部解析真实客户端 IP（`X-Forwarded-For`、`X-Real-IP`、`True-Client-Ip`）

#### NormalizePath 中间件

标准化请求路径：

- 自动处理尾部斜杠
- 统一路径格式

## 项目结构

```
src/
├── lib.rs          # 库导出模块
├── main.rs         # 主程序入口
├── server.rs       # 服务器核心逻辑
├── logger.rs       # 日志系统
├── middleware/     # 中间件模块
│   └── logger.rs   # 访问日志中间件
```

## 环境变量配置

- `HTTP_HOST`: 服务器监听地址（默认：127.0.0.1）
- `HTTP_PORT`: 服务器监听端口（默认：33720）
- `LOGGER_LEVEL`: 日志级别（默认：info）
- `LOGGER_ENABLE`: 日志启用开关（默认：info）

## 特性

- **异步支持**: 基于 tokio 异步运行时
- **高性能**: 使用 actix-web 框架提供高性能 HTTP 服务
- **可配置**: 支持环境变量和参数配置
- **日志记录**: 完整的访问日志和应用日志
- **IP 解析**: 智能解析真实客户端 IP 地址
- **路径标准化**: 自动处理 URL 路径格式

## 示例响应

服务器启动后，默认在根路径 `/` 返回 "Hello World!" 响应。

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
