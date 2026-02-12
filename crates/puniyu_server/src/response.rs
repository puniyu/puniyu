use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

/// 标准 API 响应结构
///
/// 用于统一 HTTP API 的响应格式，包含状态码、数据和消息。
///
/// # 泛型参数
///
/// - `T` - 响应数据的类型，必须实现 `Serialize` trait
///
/// # 字段
///
/// - `code` - HTTP 状态码
/// - `data` - 响应数据，可选
/// - `message` - 响应消息
///
/// # 示例
///
/// ## 成功响应
///
/// ```rust,ignore
/// use puniyu_server::BaseResponse;
///
/// let response = BaseResponse::ok("操作成功", Some(user_data));
/// response.send_json();
/// ```
///
/// ## 错误响应
///
/// ```rust,ignore
/// let response = BaseResponse::<()>::error(404, "用户不存在");
/// response.send_json();
/// ```
#[derive(Serialize, Deserialize)]
pub struct BaseResponse<T> {
    /// HTTP 状态码
    pub code: u16,
    /// 响应数据
    pub data: Option<T>,
    /// 响应消息
    pub message: String,
}

impl Default for BaseResponse<()> {
    fn default() -> Self {
        Self { code: StatusCode::NOT_FOUND.as_u16(), data: None, message: "not found".to_string() }
    }
}

#[allow(dead_code)]
impl<T> BaseResponse<T> {
    /// 创建成功响应（200 OK）
    ///
    /// # 参数
    ///
    /// - `message` - 响应消息
    /// - `data` - 响应数据
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::ok("获取成功", Some(user));
    /// ```
    pub fn ok(message: impl Into<String>, data: Option<T>) -> Self {
        Self { code: StatusCode::OK.as_u16(), data, message: message.into() }
    }

    /// 创建成功响应，仅包含数据（200 OK）
    ///
    /// # 参数
    ///
    /// - `data` - 响应数据
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::success(user);
    /// ```
    pub fn success(data: T) -> Self {
        Self { code: StatusCode::OK.as_u16(), data: Some(data), message: "success".to_string() }
    }

    /// 创建错误响应
    ///
    /// # 参数
    ///
    /// - `code` - HTTP 状态码
    /// - `message` - 错误消息
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::<()>::error(404, "资源不存在");
    /// ```
    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self { code, data: None, message: message.into() }
    }

    /// 创建 404 Not Found 响应
    ///
    /// # 参数
    ///
    /// - `message` - 错误消息
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::<()>::not_found("用户不存在");
    /// ```
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::error(StatusCode::NOT_FOUND.as_u16(), message)
    }

    /// 创建 400 Bad Request 响应
    ///
    /// # 参数
    ///
    /// - `message` - 错误消息
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::<()>::bad_request("参数错误");
    /// ```
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::error(StatusCode::BAD_REQUEST.as_u16(), message)
    }

    /// 创建 500 Internal Server Error 响应
    ///
    /// # 参数
    ///
    /// - `message` - 错误消息
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::<()>::internal_error("服务器错误");
    /// ```
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message)
    }

    /// 创建 401 Unauthorized 响应
    ///
    /// # 参数
    ///
    /// - `message` - 错误消息
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::<()>::unauthorized("未授权");
    /// ```
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::error(StatusCode::UNAUTHORIZED.as_u16(), message)
    }

    /// 创建 403 Forbidden 响应
    ///
    /// # 参数
    ///
    /// - `message` - 错误消息
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let response = BaseResponse::<()>::forbidden("无权限访问");
    /// ```
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::error(StatusCode::FORBIDDEN.as_u16(), message)
    }

    /// 将响应转换为 JSON 格式的 HttpResponse
    ///
    /// # 返回
    ///
    /// 返回包含 JSON 格式响应体的 `HttpResponse`
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// async fn handler() -> HttpResponse {
    ///     BaseResponse::ok("成功", Some(data)).send_json()
    /// }
    /// ```
    pub fn send_json(&self) -> HttpResponse
    where
        T: Serialize,
    {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        HttpResponse::build(status).content_type("application/json").body(
            serde_json::to_string_pretty(self).unwrap_or_else(|_| {
                r#"{"code":500,"data":null,"message":"Failed to serialize response"}"#.to_string()
            }),
        )
    }

}
