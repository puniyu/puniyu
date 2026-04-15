use jiff::Timestamp;
use puniyu_adapter_types as puniyu_adapter;
include!(concat!(env!("OUT_DIR"), "/puniyu.adapter.rs"));

impl_enum_from!(AdapterPlatform => puniyu_adapter::AdapterPlatform {
	Qq => QQ,
	Wechat => Wechat,
	Telegram => Telegram,
	Discord => Discord,
	Kook => Kook,
	PlatformOther => Other,
});

impl_enum_from!(AdapterStandard => puniyu_adapter::AdapterStandard {
	OneBotV11 => OneBotV11,
	OneBotV12 => OneBotV12,
	Oicq => Oicq,
	Icqq => Icqq,
	StandardOther => Other,
	Milky => Milky,
	Satori => Satori,
});

impl_enum_from!(AdapterProtocol => puniyu_adapter::AdapterProtocol {
	QqBot => QQBot,
	Oicq => Oicq,
	Icqq => Icqq,
	GoCqHttp => GoCqHttp,
	NapCat => NapCat,
	LlOneBot => LLOneBot,
	Conwechat => Conwechat,
	Lagrange => Lagrange,
	Console => Console,
	Other => Other,
	Yogurt => Yogurt,
});

impl_enum_from!(AdapterCommunication => puniyu_adapter::AdapterCommunication {
	Http => Http,
	WebSocketServer => WebSocketServer,
	WebSocketClient => WebSocketClient,
	Grpc => Grpc,
	Other => Other,
	Sse => Sse,
});

impl From<AdapterInfo> for puniyu_adapter::AdapterInfo {
	fn from(adapter: AdapterInfo) -> Self {
		let platform = AdapterPlatform::try_from(adapter.platform).unwrap_or_default();
		let standard = AdapterStandard::try_from(adapter.standard).unwrap_or_default();
		let protocol = AdapterProtocol::try_from(adapter.protocol).unwrap_or_default();
		let communication = AdapterCommunication::try_from(adapter.communication).unwrap_or_default();
		let connect_time = Timestamp::from_second(adapter.connect_time as i64).unwrap_or_else(|_| Timestamp::now());

		Self {
			name: adapter.name,
			author: adapter.author,
			version: adapter.version.map(Into::into).unwrap(),
			platform: platform.into(),
			standard: standard.into(),
			protocol: protocol.into(),
			communication: communication.into(),
			address: adapter.address,
			connect_time,
			secret: adapter.secret,
		}
	}
}

impl From<puniyu_adapter::AdapterInfo> for AdapterInfo {
	fn from(adapter: puniyu_adapter::AdapterInfo) -> Self {
		let connect_time = adapter.connect_time.as_second();
		Self {
			name: adapter.name,
			version: Some(adapter.version.into()),
			platform: AdapterPlatform::from(adapter.platform).into(),
			standard: AdapterStandard::from(adapter.standard).into(),
			protocol: AdapterProtocol::from(adapter.protocol).into(),
			communication: AdapterCommunication::from(adapter.communication).into(),
			address: adapter.address,
			connect_time: connect_time as u64,
			secret: adapter.secret,
			author: adapter.author,
		}
	}
}
