pub mod receive;
pub mod send;

pub trait Alias {
	fn alias(&self) -> String;
}
