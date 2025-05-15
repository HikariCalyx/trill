pub struct Backend {
    uris: Vec<String>,
    user: Option<String>,
    cred: Option<String>,
}

impl Backend {
    pub fn new(addr: String, user: String, cred: String) -> Self {
        Self {
            uris: vec![format!("turn:{addr}")],
            user: Some(user),
            cred: Some(cred),
        }
    }
}

#[async_trait::async_trait]
impl super::Backend for Backend {
    async fn get(
        &self,
        _remote_ip: &std::net::IpAddr,
    ) -> anyhow::Result<Vec<tango_signaling::proto::signaling::packet::hello::IceServer>> {
        Ok(vec![tango_signaling::proto::signaling::packet::hello::IceServer {
            credential: self.cred.clone(),
            username: self.user.clone(),
            urls: self.uris.clone(),
        }])
    }
}
