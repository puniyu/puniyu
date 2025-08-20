use serde::{Deserialize, Serialize};

/// 适配器平台
///
/// 用于标识适配器的平台，用于在不同平台之间进行消息传递。
/// - QQ：QQ 平台
/// - Wechat： 微信平台
/// - Telegram: Telegram 平台
/// - Discord: Discord 平台
/// - Kook: 开黑吧 平台
/// - Other: 其他平台
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdapterPlatform {
    QQ,
    Wechat,
    Telegram,
    Discord,
    Kook,
    Other,
}

/// 适配器所使用的标准接口协议
///
/// - OneBotV11: onebot v11 标准
/// - OneBotV12: onebot v12 标准
/// - OICQ: OICQ 标准
/// - ICQQ: OICQ fork 标准
/// - Other: 其他标准
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdapterStandard {
    OneBotV11,
    OneBotV12,
    OICQ,
    ICQQ,
    Other,
}

/// 适配器协议实现
///
/// 用于标识适配器所使用的协议实现，用于在不同平台之间进行消息传递。
///
/// - QQBOT: [QQ 平台协议实现](https://bot.q.qq.com/wiki)
/// - ICQQ: [OICQ 平台协议实现](https://github.com/takayama-lily/oicq)
/// - GoCqHttp: [go-cqhttp 协议实现](https://docs.go-cqhttp.org/)
/// - NapCat: [NapCat 协议实现](https://napneko.github.io/zh-CN/)
/// - LLOneBot: [LLOneBot 协议实现](https://llonebot.github.io/zh-CN/)
/// - Lagrange: [Lagrange 协议实现](ttps://lagrangedev.github.io/Lagrange.Doc/Lagrange.OneBot/)
/// - Console: 控制台协议实现
/// - Other: 其他协议实现
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdapterProtocol {
    QQBOT,
    ICQQ,
    GoCqHttp,
    NapCat,
    LLOneBot,
    Conwechat,
    Lagrange,
    Console,
    Other,
}

/// 适配器通信方式
///
/// 用于标识适配器所使用的通信方式，用于在不同平台之间进行消息传递。
///
/// - Http: Http 通信方式
/// - WebSocketServer: WebSocket 服务器通信方式
/// - WebSocketClient: WebSocket 客户端通信方式
/// - Grpc: Grpc 通信方式
/// - Other: 其他通信方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdapterCommunication {
    Http,
    WebSocketServer,
    WebSocketClient,
    Grpc,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterInfo {
    /// 适配器索引
    pub index: u64,
    /// 适配器名称 如lagrange-onebot
    pub name: String,
    /// 适配器版本
    pub version: String,
    /// 适配器平台
    pub platform: AdapterPlatform,
    /// 适配器使用的协议标准 如onebot11
    pub standard: AdapterStandard,
    /// 适配器协议实现 如gocq、napcat
    pub protocol: AdapterProtocol,
    /// 适配器通信方式
    pub communication: AdapterCommunication,
    /// 适配器通信地址
    ///
    /// # 示例
    /// `http://127.0.0.1:7000`
    /// `ws://127.0.0.1:7000/ws`
    /// `grpc://127.0.0.1:7001`
    /// `internal://127.0.0.1`
    pub address: String,
    /// 连接时间
    pub connect_time: u32,
    /// 鉴权秘钥
    pub secret: Option<String>,
}
