use rand::Rng;
use rand::distr::Alphanumeric;

pub(crate) fn make_message_id() -> String {
	rand::rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect()
}
