use bon::Builder;
use semver::Version;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use super::{Platform, Protocol, Standard};

/// 适配器元信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_adapter_api::{AdapterInfo, Platform, Protocol, Standard};
/// use semver::Version;
///
/// let info = AdapterInfo {
///     name: "onebot".into(),
///     version: Version::new(1, 0, 0),
///     platform: Platform::QQ,
///     protocol: Protocol::NapCat,
///     standard: Standard::OneBotV11,
/// };
///
/// assert_eq!(info.name, "onebot");
/// assert_eq!(info.platform, Platform::QQ);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct AdapterInfo {
	/// 适配器名称
	#[builder(into)]
	pub name: SmolStr,
	/// 适配器版本
	pub version: Version,
	/// 适配器平台
	pub platform: Platform,
	/// 适配器协议实现
	pub protocol: Protocol,
	/// 适配器标准
	pub standard: Standard,
}
