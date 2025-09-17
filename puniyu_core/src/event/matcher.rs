mod builder;

#[derive(Debug, Clone)]
pub enum MatcherType {
	Message,
	Notice,
	Request,
}
