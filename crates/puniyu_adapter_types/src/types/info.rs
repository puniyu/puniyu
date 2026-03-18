use chrono::{DateTime, Utc};
use derive_builder::Builder;
use puniyu_version::Version;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 适配器平台
///
/// 用于标识适配器的平台，用于在不同平台之间进行消息传递。
///
/// # 变体
///
/// - `QQ` - QQ 平台
/// - `Wechat` - 微信平台
/// - `Telegram` - Telegram 平台
/// - `Discord` - Discord 平台
/// - `Kook` - 开黑吧平台
/// - `Other` - 其他平台（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_adapter_core::types::info::AdapterPlatform;
///
/// let platform = AdapterPlatform::QQ;
/// assert_eq!(platform.to_string(), "qq");
///
/// let platform: AdapterPlatform = "telegram".parse().unwrap();
/// assert_eq!(platform, AdapterPlatform::Telegram);
/// ```
#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
pub enum AdapterPlatform {
	#[strum(serialize = "qq")]
	QQ,
	#[strum(serialize = "wechat")]
	Wechat,
	#[strum(serialize = "telegram")]
	Telegram,
	#[strum(serialize = "discord")]
	Discord,
	#[strum(serialize = "kook")]
	Kook,
	#[strum(serialize = "other")]
	#[default]
	Other,
}

/// 适配器所使用的标准接口协议
///
/// 定义适配器遵循的标准协议规范。
///
/// # 变体
///
/// - `OneBotV11` - [OneBot v11 标准](https://github.com/botuniverse/onebot-11)
/// - `OneBotV12` - [OneBot v12 标准](https://github.com/botuniverse/onebot-12)
/// - `Oicq` - OICQ 标准
/// - `Icqq` - ICQQ 标准（OICQ fork）
/// - `Other` - 其他标准（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_adapter_core::types::info::AdapterStandard;
///
/// let standard = AdapterStandard::OneBotV11;
/// assert_eq!(standard.to_string(), "OnebotV11");
/// ```
#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
pub enum AdapterStandard {
	#[strum(serialize = "OnebotV11")]
	OneBotV11,
	#[strum(serialize = "OnebotV12")]
	OneBotV12,
	#[strum(serialize = "Oicq")]
	Oicq,
	#[strum(serialize = "Icqq")]
	Icqq,
	#[strum(serialize = "Other")]
	#[default]
	Other,
}

/// 适配器协议实现
///
/// 用于标识适配器所使用的具体协议实现，不同的实现可能有不同的特性和功能。
///
/// # 变体
///
/// - `QQBot` - [QQ 官方机器人平台](https://bot.q.qq.com/wiki)
/// - `Icqq` - [ICQQ 协议实现](https://github.com/icqqjs/icqq)
/// - `Oicq` - [OICQ 协议实现](https://github.com/takayama-lily/oicq)
/// - `GoCqHttp` - [go-cqhttp 协议实现](https://docs.go-cqhttp.org/)
/// - `NapCat` - [NapCat 协议实现](https://napneko.github.io/zh-CN/)
/// - `LLOneBot` - [LLOneBot 协议实现](https://llonebot.github.io/zh-CN/)
/// - `Conwechat` - 微信协议实现
/// - `Lagrange` - [Lagrange 协议实现](https://lagrangedev.github.io/Lagrange.Doc/Lagrange.OneBot/)
/// - `Console` - 控制台协议实现（用于测试和调试）
/// - `Other` - 其他协议实现（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_adapter_core::types::info::AdapterProtocol;
///
/// let protocol = AdapterProtocol::NapCat;
/// assert_eq!(protocol.to_string(), "NapCat");
///
/// let protocol: AdapterProtocol = "GoCqHttp".parse().unwrap();
/// assert_eq!(protocol, AdapterProtocol::GoCqHttp);
/// ```
#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
pub enum AdapterProtocol {
	#[strum(serialize = "QQBot")]
	QQBot,
	#[strum(serialize = "Icqq")]
	Icqq,
	#[strum(serialize = "Oicq")]
	Oicq,
	#[strum(serialize = "GoCqHttp")]
	GoCqHttp,
	#[strum(serialize = "NapCat")]
	NapCat,
	#[strum(serialize = "LLOneBot")]
	LLOneBot,
	#[strum(serialize = "Conwechat")]
	Conwechat,
	#[strum(serialize = "Lagrange")]
	Lagrange,
	#[strum(serialize = "Console")]
	Console,
	#[strum(serialize = "Other")]
	#[default]
	Other,
}

/// 适配器通信方式
///
/// 用于标识适配器所使用的通信方式，决定了适配器如何与平台进行数据交互。
///
/// # 变体
///
/// - `Http` - HTTP 通信方式（RESTful API）
/// - `WebSocketServer` - WebSocket 服务器模式（适配器作为服务器）
/// - `WebSocketClient` - WebSocket 客户端模式（适配器作为客户端）
/// - `Grpc` - gRPC 通信方式
/// - `Other` - 其他通信方式（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_adapter_core::types::info::AdapterCommunication;
///
/// let comm = AdapterCommunication::WebSocketServer;
/// assert_eq!(comm.to_string(), "WebSocketServer");
///
/// let comm: AdapterCommunication = "Http".parse().unwrap();
/// assert_eq!(comm, AdapterCommunication::Http);
/// ```
///
/// # 使用场景
///
/// - `Http` - 适用于简单的请求-响应模式
/// - `WebSocketServer` - 适用于需要接收平台推送事件的场景
/// - `WebSocketClient` - 适用于主动连接到平台的场景
/// - `Grpc` - 适用于高性能、低延迟的场景
#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
pub enum AdapterCommunication {
	#[strum(serialize = "Http")]
	Http,
	#[strum(serialize = "WebSocketServer")]
	WebSocketServer,
	#[strum(serialize = "WebSocketClient")]
	WebSocketClient,
	#[strum(serialize = "Grpc")]
	Grpc,
	#[default]
	#[strum(serialize = "Other")]
	Other,
}

/// 适配器信息
///
/// 包含适配器的完整元数据，用于标识和配置适配器。
///
/// # 字段
///
/// - `name` - 适配器名称，如 "puniyu_adapter_onebot"
/// - `author` - 适配器作者（可选）
/// - `version` - 适配器版本
/// - `platform` - 适配器平台（QQ、微信等）
/// - `standard` - 适配器使用的协议标准（如 OneBot v11）
/// - `protocol` - 适配器协议实现（如 NapCat、go-cqhttp）
/// - `communication` - 适配器通信方式（HTTP、WebSocket 等）
/// - `address` - 适配器通信地址（可选）
/// - `connect_time` - 连接时间（自动设置为当前时间）
/// - `secret` - 鉴权密钥（可选）
///
/// # 示例
///
/// ## 使用 Builder 模式
///
/// ```rust
/// use puniyu_adapter_core::types::info::{
///     AdapterInfoBuilder, AdapterPlatform, AdapterProtocol, AdapterCommunication
/// };
/// use puniyu_version::Version;
///
/// let info = AdapterInfoBuilder::default()
///     .name("my_adapter")
///     .platform(AdapterPlatform::QQ)
///     .protocol(AdapterProtocol::NapCat)
///     .communication(AdapterCommunication::WebSocketServer)
///     .version(Version::new(1, 0, 0))
///     .address("127.0.0.1:8080".to_string())
///     .build()
///     .unwrap();
/// ```
///
/// ## 使用宏
///
/// ```rust
/// use puniyu_adapter_core::adapter_info;
/// use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
///
/// let info = adapter_info!(
///     "my_adapter",
///     AdapterPlatform::QQ,
///     AdapterProtocol::NapCat
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Builder, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct AdapterInfo {
	/// 适配器名称 如puniyu_adapter_onebot
	#[builder(default)]
	pub name: String,
	/// 适配器作者
	#[builder(default)]
	pub author: Option<String>,
	/// 适配器版本
	#[builder(default = "Self::default_version()")]
	pub version: Version,
	/// 适配器平台
	#[builder(default)]
	pub platform: AdapterPlatform,
	/// 适配器使用的协议标准 如onebot11
	#[builder(default)]
	pub standard: AdapterStandard,
	/// 适配器协议实现 如gocq、napcat
	#[builder(default)]
	pub protocol: AdapterProtocol,
	/// 适配器通信方式
	#[builder(default)]
	pub communication: AdapterCommunication,
	/// 适配器通信地址
	///
	/// # 示例
	/// `127.0.0.1:7000`
	/// `127.0.0.1:7000/ws`
	/// `127.0.0.1:7001`
	#[builder(default)]
	pub address: Option<String>,
	/// 连接时间
	#[builder(default = "Self::default_connect_time()")]
	pub connect_time: DateTime<Utc>,
	/// 鉴权密钥
	#[builder(default)]
	pub secret: Option<String>,
}

impl Default for AdapterInfo {
	fn default() -> Self {
		AdapterInfoBuilder::default().build().expect("Failed to build AdapterInfo")
	}
}

impl AdapterInfoBuilder {
	fn default_version() -> Version {
		Version::new(0, 1, 0)
	}
	fn default_connect_time() -> DateTime<Utc> {
		Utc::now()
	}
}

/// 创建适配器信息的便捷宏
///
/// 该宏提供了一种简洁的方式来创建 `AdapterInfo` 实例，无需手动使用 Builder 模式。
///
/// # 语法
///
/// ## 命名字段形式
///
/// ```rust,ignore
/// adapter_info!(
///     key1: value1,
///     key2: value2,
///     ...
/// )
/// ```
///
/// ## 快捷形式（三参数）
///
/// ```rust,ignore
/// adapter_info!(name, platform, protocol)
/// ```
///
/// 快捷形式会创建一个包含名称、平台和协议的基本适配器信息。
///
/// # 示例
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_adapter_core::adapter_info;
/// use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol, AdapterCommunication};
/// use puniyu_version::Version;
///
/// let info = adapter_info!(
///     name: "my_adapter",
///     platform: AdapterPlatform::QQ,
///     protocol: AdapterProtocol::NapCat,
///     communication: AdapterCommunication::WebSocketServer,
///     version: Version::new(1, 0, 0),
/// );
///
/// assert_eq!(info.name, "my_adapter");
/// assert_eq!(info.platform, AdapterPlatform::QQ);
/// ```
///
/// ## 使用快捷形式
///
/// ```rust
/// use puniyu_adapter_core::adapter_info;
/// use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
///
/// let info = adapter_info!(
///     "my_adapter",
///     AdapterPlatform::QQ,
///     AdapterProtocol::NapCat
/// );
///
/// assert_eq!(info.name, "my_adapter");
/// assert_eq!(info.platform, AdapterPlatform::QQ);
/// assert_eq!(info.protocol, AdapterProtocol::NapCat);
/// ```
///
/// ## 完整配置示例
///
/// ```rust
/// use puniyu_adapter_core::adapter_info;
/// use puniyu_adapter_core::types::info::{
///     AdapterPlatform, AdapterProtocol, AdapterStandard, AdapterCommunication
/// };
/// use puniyu_version::Version;
///
/// let info = adapter_info!(
///     name: "napcat_adapter",
///     author: Some("Puniyu Team".to_string()),
///     version: Version::new(1, 0, 0),
///     platform: AdapterPlatform::QQ,
///     standard: AdapterStandard::OneBotV11,
///     protocol: AdapterProtocol::NapCat,
///     communication: AdapterCommunication::WebSocketServer,
///     address: Some("127.0.0.1:8080".to_string()),
///     secret: Some("my_secret".to_string()),
/// );
/// ```
///
/// # 可用字段
///
/// - `name` - 适配器名称（String）
/// - `author` - 适配器作者（Option<String>）
/// - `version` - 适配器版本（Version）
/// - `platform` - 适配器平台（AdapterPlatform）
/// - `standard` - 适配器标准（AdapterStandard）
/// - `protocol` - 适配器协议（AdapterProtocol）
/// - `communication` - 通信方式（AdapterCommunication）
/// - `address` - 通信地址（Option<String>）
/// - `connect_time` - 连接时间（DateTime<Utc>）
/// - `secret` - 鉴权密钥（Option<String>）
///
/// # Panics
///
/// 如果必需字段缺失或字段值无效，宏会 panic。
#[macro_export]
macro_rules! adapter_info {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        let  mut builder = $crate::types::info::AdapterInfoBuilder::default();
		$(
			builder.$key($value);
		)*
		builder.build().expect("Failed to build AdapterInfo")
    }};
	($name:expr, $platform:expr, $protocol:expr) => {{
		adapter_info!(
			name: $name,
			platform: $platform,
			protocol: $protocol,
		)
	}};
}
