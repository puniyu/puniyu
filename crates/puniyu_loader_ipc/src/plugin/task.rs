pub struct IpcTask {
	name: Option<String>,
	cron: String,
}

impl IpcTask {
	pub fn new(name: Option<String>, cron: String) -> Self {
		Self { name, cron }
	}

    pub fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

    pub fn cron(&self) -> &str {
		&self.cron
	}
}
