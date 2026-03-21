use actix_web::{
	Error,
	body::{BoxBody, MessageBody, to_bytes},
	dev::{Service, ServiceRequest, ServiceResponse, Transform},
	error::ErrorInternalServerError,
	http::header,
};
use futures_util::future::LocalBoxFuture;
use std::fmt::{Debug, Display};
use std::future::{Ready, ready};

pub struct PrettyJson;

pub struct PrettyJsonMiddleware<S> {
	service: S,
}

impl<S, B> Transform<S, ServiceRequest> for PrettyJson
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: MessageBody + 'static,
	B::Error: Debug + Display,
{
	type Response = ServiceResponse<BoxBody>;
	type Error = Error;
	type Transform = PrettyJsonMiddleware<S>;
	type InitError = ();
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(PrettyJsonMiddleware { service }))
	}
}

impl<S, B> Service<ServiceRequest> for PrettyJsonMiddleware<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: MessageBody + 'static,
	B::Error: Debug + Display,
{
	type Response = ServiceResponse<BoxBody>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	actix_web::dev::forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let fut = self.service.call(req);

		Box::pin(async move {
			let response = fut.await?;

			let is_json = response
				.headers()
				.get(header::CONTENT_TYPE)
				.and_then(|ct| ct.to_str().ok())
				.map(|ct| ct.contains(mime::APPLICATION_JSON.as_ref()))
				.unwrap_or(false);

			if !is_json {
				return Ok(response.map_into_boxed_body());
			}

			let (req, res) = response.into_parts();

			let status = res.status();
			let mut builder = actix_web::HttpResponse::build(status);
			for (name, value) in res.headers() {
				builder.append_header((name.clone(), value.clone()));
			}

			let bytes = to_bytes(res.into_body()).await.map_err(ErrorInternalServerError)?;

			let pretty = serde_json::from_slice::<serde_json::Value>(&bytes)
				.ok()
				.and_then(|v| serde_json::to_vec_pretty(&v).ok())
				.unwrap_or_else(|| bytes.to_vec());

			Ok(ServiceResponse::new(req, builder.body(pretty).map_into_boxed_body()))
		})
	}
}
