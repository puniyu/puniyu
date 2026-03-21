use std::env::current_dir;
use clap::Parser;
use const_str::parse;
use puniyu_logger::{LoggerOptions, init as log_init};
use puniyu_common::app as puniyu_common;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Parser)]
#[command(
	name = "puniyu_server",
	about = "Puniyu 服务器",
	disable_help_flag = true,
	help_template = "{about}\n\n用法: {usage}\n\n选项:\n{options}"
)]
struct Args {
	/// 监听地址
	#[arg(short, long)]
	host: Option<IpAddr>,
	/// 监听端口
	#[arg(short, long)]
	port: Option<u16>,
	/// 显示帮助
	#[arg(long, action = clap::ArgAction::Help)]
	help: Option<bool>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let args = Args::parse();
	static VERSION: puniyu_version::Version = puniyu_version::Version::new(
		parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
		parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
		parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
	);

	let info = puniyu_common::AppInfo::new("puniyu-server", &VERSION, current_dir().unwrap());
	puniyu_common::set_app_info(info);
	log_init(Some(LoggerOptions::default().with_prefix("puniyu-server")));
	puniyu_server::run_server(
		args.host.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
		args.port.unwrap_or(33700),
	)
	.await?;
	Ok(())
}
