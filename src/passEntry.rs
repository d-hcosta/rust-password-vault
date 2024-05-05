#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
  pub service: String,
  pub username: String,
  pub password: String,
}

impl ServiceInfo {
  pub fn new(service: String, username: String, password: String) -> Self {
    ServiceInfo {
      service,
      username,
      password,
    }
  }
}