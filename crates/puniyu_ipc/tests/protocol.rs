use puniyu_ipc::{Error, Message, MessageKind, PROTOCOL_VERSION, RequestMessage, ResponseMessage};

#[test]
fn request_round_trip() {
	let message = Message::request(1, "plugin.command.ping", ["a", "b"]).expect("request");
	let frame = encode_message(&message).expect("encode frame");
	let decoded = decode_message(&frame).expect("decode frame");
	assert_eq!(decoded, message);
}

#[test]
fn response_round_trip() {
	let message = Message::response(7, vec!["pong"]).expect("response");
	let frame = encode_message(&message).expect("encode frame");
	let decoded = decode_message(&frame).expect("decode frame");
	assert_eq!(decoded, message);
}

#[test]
fn error_round_trip() {
	let message = Message::error(9, "METHOD_NOT_FOUND", "service not found");
	let frame = encode_message(&message).expect("encode frame");
	let decoded = decode_message(&frame).expect("decode frame");
	assert_eq!(decoded, message);
}

#[test]
fn cancel_round_trip() {
	let message = Message::cancel(12);
	let frame = encode_message(&message).expect("encode frame");
	let decoded = decode_message(&frame).expect("decode frame");
	assert_eq!(decoded, message);
}

#[test]
fn event_round_trip() {
	let message = Message::event("host.event.shutdown", ()).expect("event");
	let frame = encode_message(&message).expect("encode frame");
	let decoded = decode_message(&frame).expect("decode frame");
	assert_eq!(decoded, message);
}

#[test]
fn rejects_unknown_version() {
	let message = Message::cancel(1);
	let mut frame = encode_message(&message).expect("encode frame");
	frame[4] = 2;
	let error = decode_message(&frame).expect_err("unknown version should fail");
	assert!(matches!(error, Error::UnsupportedVersion(2)));
}

#[test]
fn rejects_unknown_kind() {
	let message = Message::cancel(1);
	let mut frame = encode_message(&message).expect("encode frame");
	frame[5] = 99;
	let error = decode_message(&frame).expect_err("unknown kind should fail");
	assert!(matches!(error, Error::UnknownKind(99)));
}

#[test]
fn rejects_invalid_payload() {
	let frame = vec![0, 0, 0, 3, PROTOCOL_VERSION, MessageKind::Request as u8, 0xff];
	let error = decode_message(&frame).expect_err("invalid payload should fail");
	assert!(matches!(error, Error::Decode(_)));
}

#[test]
fn request_decode_params() {
	let request = RequestMessage::new(1, "plugin.task.schedule", vec!["a", "b"]).expect("request");
	let params: Vec<String> = request.decode_params().expect("decode params");
	assert_eq!(params, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn response_decode_result() {
	let response = ResponseMessage::new(1, vec!["pong"]).expect("response");
	let result: Vec<String> = response.decode_result().expect("decode result");
	assert_eq!(result, vec!["pong".to_string()]);
}

fn encode_message(message: &Message) -> puniyu_ipc::Result<Vec<u8>> {
	let payload = match message {
		Message::Request(request) => rmp_serde::to_vec(request)?,
		Message::Response(response) => rmp_serde::to_vec(response)?,
		Message::Error(error) => rmp_serde::to_vec(error)?,
		Message::Cancel(cancel) => rmp_serde::to_vec(cancel)?,
		Message::Event(event) => rmp_serde::to_vec(event)?,
	};

	let frame_len = payload.len() + 2;
	if frame_len > puniyu_ipc::MAX_FRAME_SIZE {
		return Err(Error::FrameTooLarge(frame_len));
	}

	let mut frame = Vec::with_capacity(frame_len + 4);
	frame.extend_from_slice(&(frame_len as u32).to_be_bytes());
	frame.push(PROTOCOL_VERSION);
	frame.push(message.kind() as u8);
	frame.extend_from_slice(&payload);
	Ok(frame)
}

fn decode_message(frame: &[u8]) -> puniyu_ipc::Result<Message> {
	if frame.len() < 6 {
		return Err(Error::InvalidFrame("frame too short"));
	}

	let declared = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
	let actual = frame.len() - 4;
	if declared != actual {
		return Err(Error::FrameLengthMismatch { declared, actual });
	}
	if declared > puniyu_ipc::MAX_FRAME_SIZE {
		return Err(Error::FrameTooLarge(declared));
	}

	let version = frame[4];
	if version != PROTOCOL_VERSION {
		return Err(Error::UnsupportedVersion(version));
	}

	let kind = MessageKind::from_u8(frame[5]).ok_or(Error::UnknownKind(frame[5]))?;
	let payload = &frame[6..];

	match kind {
		MessageKind::Request => rmp_serde::from_slice(payload).map(Message::Request).map_err(Error::from),
		MessageKind::Response => rmp_serde::from_slice(payload).map(Message::Response).map_err(Error::from),
		MessageKind::Error => rmp_serde::from_slice(payload).map(Message::Error).map_err(Error::from),
		MessageKind::Cancel => rmp_serde::from_slice(payload).map(Message::Cancel).map_err(Error::from),
		MessageKind::Event => rmp_serde::from_slice(payload).map(Message::Event).map_err(Error::from),
	}
}
