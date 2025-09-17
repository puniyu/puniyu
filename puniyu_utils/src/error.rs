use thiserror::Error;
#[derive(Error, Debug)]
pub enum Config {
	#[error("配置文件错误: 读取错误")]
	Read,
	#[error("配置文件错误: 写入错误")]
	Write,
	#[error("配置文件错误: 解析错误")]
	Parse,
	#[error("配置文件错误: 删除节点错误")]
	RemoveNode,
	#[error("配置文件错误: 序列化错误")]
	Serialize,
	#[error("配置文件错误: 反序列化错误")]
	Deserialize,
	#[error("配置文件错误: 路径不存在")]
	NotFound,
	#[error("配置文件错误: 权限不足")]
	PermissionDenied,
}
