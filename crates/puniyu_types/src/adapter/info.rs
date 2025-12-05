use strum::{Display, EnumString, IntoStaticStr};
use derive_builder::Builder;

use crate::version::Version;

/// 适配器平台
///
/// 用于标识适配器的平台，用于在不同平台之间进行消息传递。
/// - QQ：QQ 平台
/// - Wechat： 微信平台
/// - Telegram: Telegram 平台
/// - Discord: Discord 平台
/// - Kook: 开黑吧 平台
/// - Other: 其他平台
#[derive(Debug, Default, Clone, PartialEq, Eq, Display, EnumString, IntoStaticStr)]
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
/// - OneBotV11: onebot v11 标准
/// - OneBotV12: onebot v12 标准
/// - OICQ: OICQ 标准
/// - ICQQ: OICQ fork 标准
/// - Other: 其他标准
#[derive(Debug, Default, Clone, PartialEq, Eq, Display, EnumString, IntoStaticStr)]
pub enum AdapterStandard {
	#[strum(serialize = "Onebot v11")]
	OneBotV11,
	#[strum(serialize = "Onebot v12")]
	OneBotV12,
	#[strum(serialize = "OICQ")]
	Oicq,
	#[strum(serialize = "ICQQ")]
	Icqq,
	#[strum(serialize = "Other")]
	#[default]
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
#[derive(Debug, Default, Clone, PartialEq, Eq, Display, EnumString, IntoStaticStr)]
pub enum AdapterProtocol {
	#[strum(serialize = "QQBot")]
	QQBot,
	#[strum(serialize = "Icqq")]
	Icqq,
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
/// 用于标识适配器所使用的通信方式，用于在不同平台之间进行消息传递。
///
/// - Http: Http 通信方式
/// - WebSocketServer: WebSocket 服务器通信方式
/// - WebSocketClient: WebSocket 客户端通信方式
/// - Grpc: Grpc 通信方式
/// - Other: 其他通信方式
#[derive(Debug, Default, Clone, PartialEq, Eq, Display, EnumString, IntoStaticStr)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into))]
/// 适配器信息
pub struct AdapterInfo {
	/// 适配器名称 如lagrange-onebot
	pub name: String,
	/// 适配器版本
	pub version: Version,
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
	/// `127.0.0.1:7000`
	/// `127.0.0.1:7000/ws`
	/// `127.0.0.1:7001`
	#[builder(default)]
	pub address: Option<String>,
	/// 连接时间
	pub connect_time: std::time::Duration,
}




/// 创建 AdapterInfo 的便捷宏
///
/// # 示例
/// ```rust,ignore
/// let info = adapter_info!(
///     name: "my_adapter",
///     version: "1.0.0",
///     platform: AdapterPlatform::QQ,
///     standard: AdapterStandard::OneBotV11,
///     protocol: AdapterProtocol::NapCat,
///     communication: AdapterCommunication::WebSocketClient,
///     connect_time: start_time
/// );
/// ```
#[cfg(feature = "adapter")]
#[macro_export]
macro_rules! adapter_info {
	(
		name: $name:expr,
		version: $version:expr,
		platform: $platform:expr,
		standard: $standard:expr,
		protocol: $protocol:expr,
		communication: $communication:expr,
		address: $address:expr,
		connect_time: $connect_time:expr
	) => {
		AdapterInfoBuilder::default()
			.name($name)
			.version($version)
			.platform($platform)
			.standard($standard)
			.protocol($protocol)
			.communication($communication)
			.address($address)
			.connect_time($connect_time)
			.build()
			.unwrap()
	};
	(
		name: $name:expr,
		version: $version:expr,
		platform: $platform:expr,
		standard: $standard:expr,
		protocol: $protocol:expr,
		communication: $communication:expr,
		connect_time: $connect_time:expr
	) => {
		AdapterInfoBuilder::default()
			.name($name)
			.version($version)
			.platform($platform)
			.standard($standard)
			.protocol($protocol)
			.communication($communication)
			.connect_time($connect_time)
			.build()
			.unwrap()
	};
	(
		name: $name:expr,
		platform: $platform:expr,
		standard: $standard:expr,
		protocol: $protocol:expr,
		communication: $communication:expr,
		address: $address:expr,
		connect_time: $connect_time:expr
	) => {
		AdapterInfoBuilder::default()
			.name($name)
			.version(env!("CARGO_PKG_VERSION"))
			.platform($platform)
			.standard($standard)
			.protocol($protocol)
			.communication($communication)
			.address($address)
			.connect_time($connect_time)
			.build()
			.unwrap()
	};
	(
		name: $name:expr,
		platform: $platform:expr,
		standard: $standard:expr,
		protocol: $protocol:expr,
		communication: $communication:expr,
		connect_time: $connect_time:expr
	) => {
		AdapterInfoBuilder::default()
			.name($name)
			.version(env!("CARGO_PKG_VERSION"))
			.platform($platform)
			.standard($standard)
			.protocol($protocol)
			.communication($communication)
			.connect_time($connect_time)
			.build()
			.unwrap()
	};
}
