use rand::Rng;
use rand::distr::Alphanumeric;

pub(crate) fn make_random_id() -> String {
	rand::rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect()
}