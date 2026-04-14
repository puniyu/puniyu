include!(concat!(env!("OUT_DIR"), "/puniyu.account.rs"));

impl From<AccountInfo> for puniyu_account::AccountInfo {
	fn from(account: AccountInfo) -> Self {
		Self { uin: account.uin, name: account.name, avatar: account.avatar }
	}
}

impl From<puniyu_account::AccountInfo> for AccountInfo {
	fn from(account: puniyu_account::AccountInfo) -> Self {
		Self { uin: account.uin, name: account.name, avatar: account.avatar }
	}
}
