use puniyu_adapter_core::adapter_info;
use puniyu_adapter_core::types::info::{
	AdapterCommunication, AdapterInfo, AdapterInfoBuilder, AdapterPlatform, AdapterProtocol,
	AdapterStandard,
};
use puniyu_version::Version;

#[test]
fn test_adapter_platform_default() {
	let platform = AdapterPlatform::default();
	assert_eq!(platform, AdapterPlatform::Other);
}

#[test]
fn test_adapter_platform_display() {
	assert_eq!(AdapterPlatform::QQ.to_string(), "qq");
	assert_eq!(AdapterPlatform::Wechat.to_string(), "wechat");
	assert_eq!(AdapterPlatform::Telegram.to_string(), "telegram");
	assert_eq!(AdapterPlatform::Discord.to_string(), "discord");
	assert_eq!(AdapterPlatform::Kook.to_string(), "kook");
	assert_eq!(AdapterPlatform::Other.to_string(), "other");
}

#[test]
fn test_adapter_platform_parse() {
	assert_eq!("qq".parse::<AdapterPlatform>().unwrap(), AdapterPlatform::QQ);
	assert_eq!("wechat".parse::<AdapterPlatform>().unwrap(), AdapterPlatform::Wechat);
	assert_eq!("telegram".parse::<AdapterPlatform>().unwrap(), AdapterPlatform::Telegram);
}

#[test]
fn test_adapter_standard_default() {
	let standard = AdapterStandard::default();
	assert_eq!(standard, AdapterStandard::Other);
}

#[test]
fn test_adapter_standard_display() {
	assert_eq!(AdapterStandard::OneBotV11.to_string(), "OnebotV11");
	assert_eq!(AdapterStandard::OneBotV12.to_string(), "OnebotV12");
	assert_eq!(AdapterStandard::Oicq.to_string(), "Oicq");
	assert_eq!(AdapterStandard::Icqq.to_string(), "Icqq");
}

#[test]
fn test_adapter_protocol_default() {
	let protocol = AdapterProtocol::default();
	assert_eq!(protocol, AdapterProtocol::Other);
}

#[test]
fn test_adapter_protocol_display() {
	assert_eq!(AdapterProtocol::QQBot.to_string(), "QQBot");
	assert_eq!(AdapterProtocol::NapCat.to_string(), "NapCat");
	assert_eq!(AdapterProtocol::GoCqHttp.to_string(), "GoCqHttp");
	assert_eq!(AdapterProtocol::Console.to_string(), "Console");
}

#[test]
fn test_adapter_protocol_parse() {
	assert_eq!("NapCat".parse::<AdapterProtocol>().unwrap(), AdapterProtocol::NapCat);
	assert_eq!("GoCqHttp".parse::<AdapterProtocol>().unwrap(), AdapterProtocol::GoCqHttp);
	assert_eq!("Console".parse::<AdapterProtocol>().unwrap(), AdapterProtocol::Console);
}

#[test]
fn test_adapter_communication_default() {
	let comm = AdapterCommunication::default();
	assert_eq!(comm, AdapterCommunication::Other);
}

#[test]
fn test_adapter_communication_display() {
	assert_eq!(AdapterCommunication::Http.to_string(), "Http");
	assert_eq!(AdapterCommunication::WebSocketServer.to_string(), "WebSocketServer");
	assert_eq!(AdapterCommunication::WebSocketClient.to_string(), "WebSocketClient");
	assert_eq!(AdapterCommunication::Grpc.to_string(), "Grpc");
}

#[test]
fn test_adapter_communication_parse() {
	assert_eq!("Http".parse::<AdapterCommunication>().unwrap(), AdapterCommunication::Http);
	assert_eq!(
		"WebSocketServer".parse::<AdapterCommunication>().unwrap(),
		AdapterCommunication::WebSocketServer
	);
}

#[test]
fn test_adapter_info_default() {
	let info = AdapterInfo::default();
	assert_eq!(info.name, "");
	assert_eq!(info.platform, AdapterPlatform::Other);
	assert_eq!(info.protocol, AdapterProtocol::Other);
}

#[test]
fn test_adapter_info_builder_minimal() {
	let info = AdapterInfoBuilder::default().name("test_adapter").build().unwrap();

	assert_eq!(info.name, "test_adapter");
	assert_eq!(info.platform, AdapterPlatform::Other);
}

#[test]
fn test_adapter_info_builder_full() {
	let info = AdapterInfoBuilder::default()
		.name("napcat_adapter")
		.author(Some("Puniyu Team".to_string()))
		.version(Version::new(1, 0, 0))
		.platform(AdapterPlatform::QQ)
		.standard(AdapterStandard::OneBotV11)
		.protocol(AdapterProtocol::NapCat)
		.communication(AdapterCommunication::WebSocketServer)
		.address(Some("127.0.0.1:8080".to_string()))
		.secret(Some("my_secret".to_string()))
		.build()
		.unwrap();

	assert_eq!(info.name, "napcat_adapter");
	assert_eq!(info.author, Some("Puniyu Team".to_string()));
	assert_eq!(info.version, Version::new(1, 0, 0));
	assert_eq!(info.platform, AdapterPlatform::QQ);
	assert_eq!(info.standard, AdapterStandard::OneBotV11);
	assert_eq!(info.protocol, AdapterProtocol::NapCat);
	assert_eq!(info.communication, AdapterCommunication::WebSocketServer);
	assert_eq!(info.address, Some("127.0.0.1:8080".to_string()));
	assert_eq!(info.secret, Some("my_secret".to_string()));
}

#[test]
fn test_adapter_info_macro_short_form() {
	let info = adapter_info!("test_adapter", AdapterPlatform::QQ, AdapterProtocol::NapCat);

	assert_eq!(info.name, "test_adapter");
	assert_eq!(info.platform, AdapterPlatform::QQ);
	assert_eq!(info.protocol, AdapterProtocol::NapCat);
}

#[test]
fn test_adapter_info_macro_named_fields() {
	let info = adapter_info!(
		name: "test_adapter",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::NapCat,
		version: Version::new(1, 0, 0),
	);

	assert_eq!(info.name, "test_adapter");
	assert_eq!(info.platform, AdapterPlatform::QQ);
	assert_eq!(info.protocol, AdapterProtocol::NapCat);
	assert_eq!(info.version, Version::new(1, 0, 0));
}

#[test]
fn test_adapter_info_clone() {
	let info1 = adapter_info!("test", AdapterPlatform::QQ, AdapterProtocol::Console);
	let info2 = info1.clone();

	assert_eq!(info1, info2);
}

#[test]
fn test_adapter_info_equality() {
	let info1 = adapter_info!(
		name: "test",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::Console,
	);
	let info2 = adapter_info!(
		name: "test",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::Console,
	);

	// 比较除了 connect_time 之外的字段
	assert_eq!(info1.name, info2.name);
	assert_eq!(info1.platform, info2.platform);
	assert_eq!(info1.protocol, info2.protocol);
}

#[test]
fn test_adapter_info_inequality() {
	let info1 = adapter_info!("test1", AdapterPlatform::QQ, AdapterProtocol::Console);
	let info2 = adapter_info!("test2", AdapterPlatform::QQ, AdapterProtocol::Console);

	assert_ne!(info1, info2);
}

#[test]
fn test_adapter_platform_clone() {
	let platform1 = AdapterPlatform::QQ;
	let platform2 = platform1.clone();
	assert_eq!(platform1, platform2);
}

#[test]
fn test_adapter_protocol_clone() {
	let protocol1 = AdapterProtocol::NapCat;
	let protocol2 = protocol1.clone();
	assert_eq!(protocol1, protocol2);
}

#[test]
fn test_adapter_communication_clone() {
	let comm1 = AdapterCommunication::WebSocketServer;
	let comm2 = comm1.clone();
	assert_eq!(comm1, comm2);
}

#[test]
fn test_adapter_info_debug() {
	let info = adapter_info!("test", AdapterPlatform::QQ, AdapterProtocol::Console);
	let debug_str = format!("{:?}", info);

	assert!(debug_str.contains("AdapterInfo"));
	assert!(debug_str.contains("test"));
}

#[test]
fn test_adapter_info_with_address() {
	let info = adapter_info!(
		name: "test",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::NapCat,
		address: Some("127.0.0.1:8080".to_string()),
	);

	assert_eq!(info.address, Some("127.0.0.1:8080".to_string()));
}

#[test]
fn test_adapter_info_without_address() {
	let info = adapter_info!("test", AdapterPlatform::QQ, AdapterProtocol::Console);

	assert_eq!(info.address, None);
}

#[test]
fn test_adapter_info_with_secret() {
	let info = adapter_info!(
		name: "test",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::NapCat,
		secret: Some("my_secret".to_string()),
	);

	assert_eq!(info.secret, Some("my_secret".to_string()));
}

#[test]
fn test_adapter_info_connect_time() {
	let info = adapter_info!("test", AdapterPlatform::QQ, AdapterProtocol::Console);

	// 连接时间应该被自动设置
	assert!(info.connect_time.timestamp() > 0);
}

#[test]
fn test_multiple_platforms() {
	let platforms = vec![
		AdapterPlatform::QQ,
		AdapterPlatform::Wechat,
		AdapterPlatform::Telegram,
		AdapterPlatform::Discord,
		AdapterPlatform::Kook,
		AdapterPlatform::Other,
	];

	for platform in platforms {
		let info = adapter_info!("test", platform.clone(), AdapterProtocol::Console);
		assert_eq!(info.platform, platform);
	}
}

#[test]
fn test_multiple_protocols() {
	let protocols = vec![
		AdapterProtocol::QQBot,
		AdapterProtocol::NapCat,
		AdapterProtocol::GoCqHttp,
		AdapterProtocol::Console,
		AdapterProtocol::Other,
	];

	for protocol in protocols {
		let info = adapter_info!("test", AdapterPlatform::QQ, protocol.clone());
		assert_eq!(info.protocol, protocol);
	}
}
