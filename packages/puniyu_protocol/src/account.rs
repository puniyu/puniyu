use puniyu_types::account::AccountInfo as puniyu_account;

include!(concat!(env!("OUT_DIR"), "/puniyu.account.rs"));

impl From<AccountInfo> for puniyu_account {
	fn from(account: AccountInfo) -> puniyu_account {
		Self { uin: account.uin, name: account.name, avatar: account.avatar }
	}
}

impl From<puniyu_account> for AccountInfo {
	fn from(account: puniyu_account) -> AccountInfo {
		Self { uin: account.uin, name: account.name, avatar: account.avatar }
	}
}
