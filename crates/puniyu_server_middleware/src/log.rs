use std::{net::IpAddr, time::Instant};

use salvo::{Depot, FlowCtrl, Handler, Request, Response, async_trait, http::HeaderMap};

pub struct AccessLog;

macro_rules! info {
    ($($arg:tt)*) => {
        {
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::log::info!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

#[async_trait]
impl Handler for AccessLog {
	async fn handle(
		&self,
		req: &mut Request,
		depot: &mut Depot,
		res: &mut Response,
		ctrl: &mut FlowCtrl,
	) {
		let start = Instant::now();
		ctrl.call_next(req, depot, res).await;
		let duration = start.elapsed();
		let method = req.method();
		let path = req.uri().path();
		let status_code =
			res.status_code.map(|s| s.as_u16().to_string()).unwrap_or("unknown".to_string());
		let headers = req.headers();
		let ip_str = parse_ip(headers)
			.or_else(|| req.remote_addr().ip())
			.map(|s| s.to_string())
			.unwrap_or_else(|| "unknown".into());

		info!("{} | {} | {} | {}ms | {}", method, path, status_code, duration.as_millis(), ip_str);
	}
}

fn parse_ip(req: &HeaderMap) -> Option<IpAddr> {
	client_ip::x_real_ip(req).ok()
}
