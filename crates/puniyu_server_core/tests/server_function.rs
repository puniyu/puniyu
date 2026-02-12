use actix_web::web::{ServiceConfig, resource, scope};
use actix_web::{HttpResponse, test};
use puniyu_server_core::ServerFunction;
use std::sync::Arc;

#[tokio::test]
async fn test_server_function_creation() {
	let _config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(resource("/test").to(|| async { HttpResponse::Ok().body("test") }));
	});
}

#[tokio::test]
async fn test_server_function_is_send_sync() {
	fn assert_send_sync<T: Send + Sync>() {}
	assert_send_sync::<ServerFunction>();
}

#[actix_web::test]
async fn test_server_function_basic_route() {
	let config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(resource("/hello").to(|| async { HttpResponse::Ok().body("Hello") }));
	});

	let config_clone = config.clone();
	let app =
		test::init_service(actix_web::App::new().configure(move |cfg| config_clone(cfg))).await;

	let req = test::TestRequest::get().uri("/hello").to_request();
	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_server_function_multiple_routes() {
	let config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(resource("/route1").to(|| async { HttpResponse::Ok().body("Route 1") }));
		cfg.service(resource("/route2").to(|| async { HttpResponse::Ok().body("Route 2") }));
	});

	let config_clone = config.clone();
	let app =
		test::init_service(actix_web::App::new().configure(move |cfg| config_clone(cfg))).await;

	let req1 = test::TestRequest::get().uri("/route1").to_request();
	let resp1 = test::call_service(&app, req1).await;
	assert!(resp1.status().is_success());

	let req2 = test::TestRequest::get().uri("/route2").to_request();
	let resp2 = test::call_service(&app, req2).await;
	assert!(resp2.status().is_success());
}

#[actix_web::test]
async fn test_server_function_with_scope() {
	let config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(
			scope("/api")
				.service(resource("/test").to(|| async { HttpResponse::Ok().body("API Test") })),
		);
	});

	let config_clone = config.clone();
	let app =
		test::init_service(actix_web::App::new().configure(move |cfg| config_clone(cfg))).await;

	let req = test::TestRequest::get().uri("/api/test").to_request();
	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_server_function_composition() {
	let config1: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(resource("/route1").to(|| async { HttpResponse::Ok().body("1") }));
	});

	let config2: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(resource("/route2").to(|| async { HttpResponse::Ok().body("2") }));
	});

	let configs = vec![config1, config2];

	let combined: ServerFunction = Arc::new(move |cfg: &mut ServiceConfig| {
		for config in &configs {
			config(cfg);
		}
	});

	// 测试组合配置
	let app = test::init_service(actix_web::App::new().configure(move |cfg| combined(cfg))).await;

	let req1 = test::TestRequest::get().uri("/route1").to_request();
	let resp1 = test::call_service(&app, req1).await;
	assert!(resp1.status().is_success());

	let req2 = test::TestRequest::get().uri("/route2").to_request();
	let resp2 = test::call_service(&app, req2).await;
	assert!(resp2.status().is_success());
}

#[tokio::test]
async fn test_server_function_conditional() {
	let enable_debug = true;

	let _config: ServerFunction = Arc::new(move |cfg: &mut ServiceConfig| {
		if enable_debug {
			cfg.service(resource("/debug").to(|| async { HttpResponse::Ok().body("Debug") }));
		}
		cfg.service(resource("/main").to(|| async { HttpResponse::Ok().body("Main") }));
	});

	// 配置函数已创建，可以在 App 中使用
}

#[actix_web::test]
async fn test_server_function_in_struct() {
	struct Server {
		config: ServerFunction,
	}

	impl Server {
		fn new(config: ServerFunction) -> Self {
			Self { config }
		}
	}

	let config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(resource("/test").to(|| async { HttpResponse::Ok().await }));
	});

	let server = Server::new(config.clone());

	// 在实际的 App 中使用
	let app =
		test::init_service(actix_web::App::new().configure(move |cfg| (server.config)(cfg))).await;

	let req = test::TestRequest::get().uri("/test").to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_server_function_empty_config() {
	let config: ServerFunction = Arc::new(|_cfg: &mut ServiceConfig| {
		// 空配置
	});

	let config_clone = config.clone();
	let _app =
		test::init_service(actix_web::App::new().configure(move |cfg| config_clone(cfg))).await;
}

#[actix_web::test]
async fn test_server_function_can_be_called_multiple_times() {
	// 测试 Fn 可以被多次调用的特性
	let config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
		cfg.service(resource("/test").to(|| async { HttpResponse::Ok().body("Test") }));
	});

	// 第一次使用
	let config_clone1 = config.clone();
	let app1 =
		test::init_service(actix_web::App::new().configure(move |cfg| config_clone1(cfg))).await;

	let req1 = test::TestRequest::get().uri("/test").to_request();
	let resp1 = test::call_service(&app1, req1).await;
	assert!(resp1.status().is_success());

	// 第二次使用（证明 Fn 可以多次调用）
	let config_clone2 = config.clone();
	let app2 =
		test::init_service(actix_web::App::new().configure(move |cfg| config_clone2(cfg))).await;

	let req2 = test::TestRequest::get().uri("/test").to_request();
	let resp2 = test::call_service(&app2, req2).await;
	assert!(resp2.status().is_success());
}
