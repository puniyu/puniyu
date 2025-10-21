# puniyu_adapter_api

puniyu 的适配器API，用于定义与不同平台通信的统一接口。

## 功能概述

该 crate 提供了统一的适配器接口，允许 puniyu 与不同的聊天平台进行交互。它定义了发送消息和获取用户/群组头像等基本操作。

## 核心组件

### AdapterApi trait

`AdapterApi` trait 定义了适配器必须实现的核心方法：

- `send_msg`: 发送消息到指定联系人
- `get_avatar_url`: 获取用户头像URL
- `get_group_avatar_url`: 获取群组头像URL

## 使用说明

适配器开发者需要实现 `AdapterApi` trait 来创建特定平台的适配器。每个方法都需要提供具体的实现来处理与目标平台的交互。

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
