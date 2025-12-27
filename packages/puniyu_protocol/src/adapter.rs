use puniyu_types::adapter as puniyu_adapter;

include!(concat!(env!("OUT_DIR"), "/puniyu.adapter.rs"));

impl From<AdapterPlatform> for puniyu_adapter::AdapterPlatform {
	fn from(adapter: AdapterPlatform) -> Self {
		match adapter {
			AdapterPlatform::Qq => puniyu_adapter::AdapterPlatform::QQ,
			AdapterPlatform::WeChat => puniyu_adapter::AdapterPlatform::Wechat,
			AdapterPlatform::Telegram => puniyu_adapter::AdapterPlatform::Telegram,
			AdapterPlatform::Discord => puniyu_adapter::AdapterPlatform::Discord,
			AdapterPlatform::Kook => puniyu_adapter::AdapterPlatform::Kook,
			AdapterPlatform::PlatformOther => puniyu_adapter::AdapterPlatform::Other,
		}
	}
}

impl From<puniyu_adapter::AdapterPlatform> for AdapterPlatform {
	fn from(adapter: puniyu_adapter::AdapterPlatform) -> Self {
		match adapter {
			puniyu_adapter::AdapterPlatform::QQ => AdapterPlatform::Qq,
			puniyu_adapter::AdapterPlatform::Wechat => AdapterPlatform::WeChat,
			puniyu_adapter::AdapterPlatform::Telegram => AdapterPlatform::Telegram,
			puniyu_adapter::AdapterPlatform::Discord => AdapterPlatform::Discord,
			puniyu_adapter::AdapterPlatform::Kook => AdapterPlatform::Kook,
			puniyu_adapter::AdapterPlatform::Other => AdapterPlatform::PlatformOther,
		}
	}
}

impl From<AdapterStandard> for puniyu_adapter::AdapterStandard {
	fn from(adapter: AdapterStandard) -> Self {
		match adapter {
			AdapterStandard::OneBotV11 => puniyu_adapter::AdapterStandard::OneBotV11,
			AdapterStandard::OneBotV12 => puniyu_adapter::AdapterStandard::OneBotV12,
			AdapterStandard::Oicq => puniyu_adapter::AdapterStandard::Oicq,
			AdapterStandard::Icqq => puniyu_adapter::AdapterStandard::Icqq,
			AdapterStandard::StandardOther => puniyu_adapter::AdapterStandard::Other,
		}
	}
}

impl From<puniyu_adapter::AdapterStandard> for AdapterStandard {
	fn from(adapter: puniyu_adapter::AdapterStandard) -> Self {
		match adapter {
			puniyu_adapter::AdapterStandard::OneBotV11 => AdapterStandard::OneBotV11,
			puniyu_adapter::AdapterStandard::OneBotV12 => AdapterStandard::OneBotV12,
			puniyu_adapter::AdapterStandard::Oicq => AdapterStandard::Oicq,
			puniyu_adapter::AdapterStandard::Icqq => AdapterStandard::Icqq,
			puniyu_adapter::AdapterStandard::Other => AdapterStandard::StandardOther,
		}
	}
}

impl From<AdapterProtocol> for puniyu_adapter::AdapterProtocol {
	fn from(adapter: AdapterProtocol) -> Self {
		match adapter {
			AdapterProtocol::QqBot => puniyu_adapter::AdapterProtocol::QQBot,
			AdapterProtocol::Icqq => puniyu_adapter::AdapterProtocol::Icqq,
			AdapterProtocol::GoCqHttp => puniyu_adapter::AdapterProtocol::GoCqHttp,
			AdapterProtocol::NapCat => puniyu_adapter::AdapterProtocol::NapCat,
			AdapterProtocol::LlOneBot => puniyu_adapter::AdapterProtocol::LLOneBot,
			AdapterProtocol::Conwechat => puniyu_adapter::AdapterProtocol::Conwechat,
			AdapterProtocol::Lagrange => puniyu_adapter::AdapterProtocol::Lagrange,
			AdapterProtocol::Console => puniyu_adapter::AdapterProtocol::Console,
			AdapterProtocol::Other => puniyu_adapter::AdapterProtocol::Other,
		}
	}
}

impl From<puniyu_adapter::AdapterProtocol> for AdapterProtocol {
	fn from(adapter: puniyu_adapter::AdapterProtocol) -> Self {
		match adapter {
			puniyu_adapter::AdapterProtocol::QQBot => AdapterProtocol::QqBot,
			puniyu_adapter::AdapterProtocol::Icqq => AdapterProtocol::Icqq,
			puniyu_adapter::AdapterProtocol::GoCqHttp => AdapterProtocol::GoCqHttp,
			puniyu_adapter::AdapterProtocol::NapCat => AdapterProtocol::NapCat,
			puniyu_adapter::AdapterProtocol::LLOneBot => AdapterProtocol::LlOneBot,
			puniyu_adapter::AdapterProtocol::Conwechat => AdapterProtocol::Conwechat,
			puniyu_adapter::AdapterProtocol::Lagrange => AdapterProtocol::Lagrange,
			puniyu_adapter::AdapterProtocol::Console => AdapterProtocol::Console,
			puniyu_adapter::AdapterProtocol::Other => AdapterProtocol::Other,
		}
	}
}

impl From<AdapterCommunication> for puniyu_adapter::AdapterCommunication {
	fn from(adapter: AdapterCommunication) -> Self {
		match adapter {
			AdapterCommunication::Http => puniyu_adapter::AdapterCommunication::Http,
			AdapterCommunication::WebSocketServer => {
				puniyu_adapter::AdapterCommunication::WebSocketServer
			}
			AdapterCommunication::WebSocketClient => {
				puniyu_adapter::AdapterCommunication::WebSocketClient
			}
			AdapterCommunication::Grpc => puniyu_adapter::AdapterCommunication::Grpc,
			AdapterCommunication::Other => puniyu_adapter::AdapterCommunication::Other,
		}
	}
}

impl From<puniyu_adapter::AdapterCommunication> for AdapterCommunication {
	fn from(adapter: puniyu_adapter::AdapterCommunication) -> Self {
		match adapter {
			puniyu_adapter::AdapterCommunication::Http => AdapterCommunication::Http,
			puniyu_adapter::AdapterCommunication::WebSocketServer => {
				AdapterCommunication::WebSocketServer
			}
			puniyu_adapter::AdapterCommunication::WebSocketClient => {
				AdapterCommunication::WebSocketClient
			}
			puniyu_adapter::AdapterCommunication::Grpc => AdapterCommunication::Grpc,
			puniyu_adapter::AdapterCommunication::Other => AdapterCommunication::Other,
		}
	}
}

impl From<AdapterInfo> for puniyu_adapter::AdapterInfo {
	fn from(adapter: AdapterInfo) -> Self {
		let platform = AdapterPlatform::try_from(adapter.platform).unwrap();
		let standard = AdapterStandard::try_from(adapter.standard).unwrap();
		let protocol = AdapterProtocol::try_from(adapter.protocol).unwrap();
		let communication = AdapterCommunication::try_from(adapter.communication).unwrap();
		let connect_time = std::time::SystemTime::UNIX_EPOCH
			+ std::time::Duration::from_secs(adapter.connect_time);

		Self {
			name: adapter.name,
			version: adapter.version.into(),
			platform: platform.into(),
			standard: standard.into(),
			protocol: protocol.into(),
			communication: communication.into(),
			address: adapter.address,
			connect_time: connect_time.into(),
		}
	}
}

impl From<puniyu_adapter::AdapterInfo> for AdapterInfo {
	fn from(adapter: puniyu_adapter::AdapterInfo) -> Self {
		let connect_time = adapter.connect_time.unix_timestamp();
		Self {
			name: adapter.name,
			version: adapter.version.into(),
			platform: AdapterPlatform::from(adapter.platform).into(),
			standard: AdapterStandard::from(adapter.standard).into(),
			protocol: AdapterProtocol::from(adapter.protocol).into(),
			communication: AdapterCommunication::from(adapter.communication).into(),
			address: adapter.address,
			connect_time: connect_time as u64,
		}
	}
}
