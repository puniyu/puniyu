use chrono::{DateTime, Utc};
use puniyu_types::adapter::{
	AdapterCommunication, AdapterInfo, AdapterInfoBuilder, AdapterPlatform, AdapterProtocol,
	AdapterStandard,
};
use puniyu_types::version::Version;
use std::str::FromStr;

#[test]
fn test_adapter_platform_enum() {
	assert_eq!(AdapterPlatform::default(), AdapterPlatform::Other);
	assert_eq!(AdapterPlatform::QQ.to_string(), "qq");
	assert_eq!(AdapterPlatform::Wechat.to_string(), "wechat");
	assert_eq!(AdapterPlatform::Telegram.to_string(), "telegram");
	assert_eq!(AdapterPlatform::Discord.to_string(), "discord");
	assert_eq!(AdapterPlatform::Kook.to_string(), "kook");
	assert_eq!(AdapterPlatform::Other.to_string(), "other");
	assert_eq!(AdapterPlatform::from_str("qq").unwrap(), AdapterPlatform::QQ);
	assert_eq!(AdapterPlatform::from_str("wechat").unwrap(), AdapterPlatform::Wechat);
	assert_eq!(AdapterPlatform::from_str("telegram").unwrap(), AdapterPlatform::Telegram);
	assert_eq!(AdapterPlatform::from_str("discord").unwrap(), AdapterPlatform::Discord);
	assert_eq!(AdapterPlatform::from_str("kook").unwrap(), AdapterPlatform::Kook);
	assert_eq!(AdapterPlatform::from_str("other").unwrap(), AdapterPlatform::Other);
	assert!(AdapterPlatform::from_str("invalid").is_err());
}

#[test]
fn test_adapter_standard_enum() {
	assert_eq!(AdapterStandard::default(), AdapterStandard::Other);
	assert_eq!(AdapterStandard::OneBotV11.to_string(), "OnebotV11");
	assert_eq!(AdapterStandard::OneBotV12.to_string(), "OnebotV12");
	assert_eq!(AdapterStandard::Oicq.to_string(), "Oicq");
	assert_eq!(AdapterStandard::Icqq.to_string(), "Icqq");
	assert_eq!(AdapterStandard::Other.to_string(), "Other");
	assert_eq!(AdapterStandard::from_str("OnebotV11").unwrap(), AdapterStandard::OneBotV11);
	assert_eq!(AdapterStandard::from_str("OnebotV12").unwrap(), AdapterStandard::OneBotV12);
	assert_eq!(AdapterStandard::from_str("Oicq").unwrap(), AdapterStandard::Oicq);
	assert_eq!(AdapterStandard::from_str("Icqq").unwrap(), AdapterStandard::Icqq);
	assert_eq!(AdapterStandard::from_str("Other").unwrap(), AdapterStandard::Other);
	assert!(AdapterStandard::from_str("invalid").is_err());
}

#[test]
fn test_adapter_protocol_enum() {
	assert_eq!(AdapterProtocol::default(), AdapterProtocol::Other);
	assert_eq!(AdapterProtocol::QQBot.to_string(), "QQBot");
	assert_eq!(AdapterProtocol::Icqq.to_string(), "Icqq");
	assert_eq!(AdapterProtocol::GoCqHttp.to_string(), "GoCqHttp");
	assert_eq!(AdapterProtocol::NapCat.to_string(), "NapCat");
	assert_eq!(AdapterProtocol::LLOneBot.to_string(), "LLOneBot");
	assert_eq!(AdapterProtocol::Conwechat.to_string(), "Conwechat");
	assert_eq!(AdapterProtocol::Lagrange.to_string(), "Lagrange");
	assert_eq!(AdapterProtocol::Console.to_string(), "Console");
	assert_eq!(AdapterProtocol::Other.to_string(), "Other");
	assert_eq!(AdapterProtocol::from_str("QQBot").unwrap(), AdapterProtocol::QQBot);
	assert_eq!(AdapterProtocol::from_str("Icqq").unwrap(), AdapterProtocol::Icqq);
	assert_eq!(AdapterProtocol::from_str("GoCqHttp").unwrap(), AdapterProtocol::GoCqHttp);
	assert_eq!(AdapterProtocol::from_str("NapCat").unwrap(), AdapterProtocol::NapCat);
	assert_eq!(AdapterProtocol::from_str("LLOneBot").unwrap(), AdapterProtocol::LLOneBot);
	assert_eq!(AdapterProtocol::from_str("Conwechat").unwrap(), AdapterProtocol::Conwechat);
	assert_eq!(AdapterProtocol::from_str("Lagrange").unwrap(), AdapterProtocol::Lagrange);
	assert_eq!(AdapterProtocol::from_str("Console").unwrap(), AdapterProtocol::Console);
	assert_eq!(AdapterProtocol::from_str("Other").unwrap(), AdapterProtocol::Other);
	assert!(AdapterProtocol::from_str("invalid").is_err());
}

#[test]
fn test_adapter_communication_enum() {
	assert_eq!(AdapterCommunication::default(), AdapterCommunication::Other);
	assert_eq!(AdapterCommunication::Http.to_string(), "Http");
	assert_eq!(AdapterCommunication::WebSocketServer.to_string(), "WebSocketServer");
	assert_eq!(AdapterCommunication::WebSocketClient.to_string(), "WebSocketClient");
	assert_eq!(AdapterCommunication::Grpc.to_string(), "Grpc");
	assert_eq!(AdapterCommunication::Other.to_string(), "Other");
	assert_eq!(AdapterCommunication::from_str("Http").unwrap(), AdapterCommunication::Http);
	assert_eq!(
		AdapterCommunication::from_str("WebSocketServer").unwrap(),
		AdapterCommunication::WebSocketServer
	);
	assert_eq!(
		AdapterCommunication::from_str("WebSocketClient").unwrap(),
		AdapterCommunication::WebSocketClient
	);
	assert_eq!(AdapterCommunication::from_str("Grpc").unwrap(), AdapterCommunication::Grpc);
	assert_eq!(AdapterCommunication::from_str("Other").unwrap(), AdapterCommunication::Other);
	assert!(AdapterCommunication::from_str("invalid").is_err());
}

#[test]
fn test_adapter_info_default() {
	let info = AdapterInfo::default();
	assert_eq!(info.name, String::new());
	assert_eq!(info.version, Version::default());
	assert_eq!(info.platform, AdapterPlatform::Other);
	assert_eq!(info.standard, AdapterStandard::Other);
	assert_eq!(info.protocol, AdapterProtocol::Other);
	assert_eq!(info.communication, AdapterCommunication::Other);
	assert_eq!(info.address, None);
	assert!(info.connect_time <= Utc::now());
}
#[test]
fn test_adapter_info_with_values() {
	let custom_time =
		DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap().with_timezone(&Utc);
	let version = Version { major: 1, minor: 2, patch: 3 };

	let info = AdapterInfoBuilder::default()
		.name("test_adapter")
		.version(version.clone())
		.platform(AdapterPlatform::QQ)
		.standard(AdapterStandard::OneBotV11)
		.protocol(AdapterProtocol::NapCat)
		.communication(AdapterCommunication::WebSocketClient)
		.address(Some("127.0.0.1:8080".to_string()))
		.connect_time(custom_time)
		.build()
		.unwrap();

	assert_eq!(info.name, "test_adapter");
	assert_eq!(info.version, version);
	assert_eq!(info.platform, AdapterPlatform::QQ);
	assert_eq!(info.standard, AdapterStandard::OneBotV11);
	assert_eq!(info.protocol, AdapterProtocol::NapCat);
	assert_eq!(info.communication, AdapterCommunication::WebSocketClient);
	assert_eq!(info.address, Some("127.0.0.1:8080".to_string()));
	assert_eq!(info.connect_time, custom_time);
}

#[cfg(feature = "adapter")]
#[test]
fn test_adapter_info_macro() {
	use puniyu_types::adapter_info;

	let info = adapter_info!(
		name: "macro_test",
		version: Version {
			major: 2,
			minor: 0,
			patch: 0,
		},
		platform: AdapterPlatform::QQ,
		standard: AdapterStandard::OneBotV11,
		protocol: AdapterProtocol::NapCat,
		communication: AdapterCommunication::WebSocketClient,
		address: Some("127.0.0.1:8080".to_string())
	);

	assert_eq!(info.name, "macro_test");
	assert_eq!(info.version, Version { major: 2, minor: 0, patch: 0 });
	assert_eq!(info.platform, AdapterPlatform::QQ);
	assert_eq!(info.standard, AdapterStandard::OneBotV11);
	assert_eq!(info.protocol, AdapterProtocol::NapCat);
	assert_eq!(info.communication, AdapterCommunication::WebSocketClient);
	assert_eq!(info.address, Some("127.0.0.1:8080".to_string()));
}
