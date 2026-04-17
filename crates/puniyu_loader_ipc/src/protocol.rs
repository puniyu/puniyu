use std::io;

use uuid::Uuid;

use crate::types::ProcessMeta;

pub(crate) const PROCESS_META_SERVICE: &str = "puniyu.plugin.meta";

pub(crate) fn build_process_meta_request() -> io::Result<puniyu_ipc::Message> {
	let request_id = Uuid::new_v4().as_u64_pair().0;
	puniyu_ipc::Message::request(request_id, PROCESS_META_SERVICE, ()).map_err(protocol_error)
}

pub(crate) fn decode_process_meta_response(
	response: puniyu_ipc::Message,
) -> io::Result<ProcessMeta> {
	match response {
		puniyu_ipc::Message::Response(response) => response.decode_result().map_err(protocol_error),
		puniyu_ipc::Message::Error(error) => {
			Err(io::Error::other(format!("{}: {}", error.error.code, error.error.message)))
		}
		other => Err(io::Error::new(
			io::ErrorKind::InvalidData,
			format!("unexpected message kind: {:?}", other.kind()),
		)),
	}
}

pub(crate) fn protocol_error(error: impl std::error::Error + Send + Sync + 'static) -> io::Error {
	io::Error::new(io::ErrorKind::InvalidData, error)
}
