pub struct Language {
    pub ping: &'static str,
    pub pong: &'static str,
    pub about: &'static str,
}

impl Language {
    pub fn ping(&self) -> String {
        self.ping.to_owned()
    }
    pub fn pong(&self, latency: &str) -> String {
        self.pong.to_owned().replace("{latency}", latency)
    }
    pub fn about(&self) -> String {
        self.about.to_owned()
    }
}
