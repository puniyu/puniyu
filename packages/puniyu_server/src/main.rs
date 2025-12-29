use clap::Parser;
use puniyu_logger::{LoggerOptions, init};
use std::{net::IpAddr};

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
	init(Some(LoggerOptions::default()));
	puniyu_server::run_server_with_control(args.host, args.port).await?;
	Ok(())
}
