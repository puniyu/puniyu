use super::send::Elements;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message(Vec<Elements>);

impl fmt::Display for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			let segments: Vec<String> = self.0.iter().map(|s| format!("{:#}", s)).collect();
			write!(f, "{}", segments.join("\n"))
		} else {
			let segments: Vec<String> = self.0.iter().map(|s| s.to_string()).collect();
			write!(f, "{}", segments.join(""))
		}
	}
}

impl From<Vec<Elements>> for Message {
	fn from(message: Vec<Elements>) -> Self {
		Message(message)
	}
}

impl From<Elements> for Message {
	fn from(elements: Elements) -> Self {
		Message(vec![elements])
	}
}

impl From<Message> for Vec<Elements> {
	fn from(message: Message) -> Self {
		message.0
	}
}

impl From<&str> for Message {
	fn from(v: &str) -> Self {
		Message(vec![Elements::Text(v.to_string().into())])
	}
}

impl From<String> for Message {
	fn from(v: String) -> Self {
		Message(vec![Elements::Text(v.into())])
	}
}

impl From<&String> for Message {
	fn from(v: &String) -> Self {
		Message(vec![Elements::Text(v.into())])
	}
}
