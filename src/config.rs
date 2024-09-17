use configparser::ini::Ini;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_address: String,
    pub server_port: u16,
    pub server_password: Option<String>,
    pub jwt_secret: Option<String>,
    pub jwt_issuer: Option<String>,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Ini::new();
        config.load(file_path).expect("Failed to load config file");

        Ok(Config {
            server_address: config
                .get("server", "address")
                .ok_or("Missing server address")?,
            server_port: config
                .get("server", "port")
                .ok_or("Missing server port")?
                .parse()?,
            server_password: config.get("server", "password").or(None),
            jwt_secret: config.get("jwt", "secret").or(None),
            jwt_issuer: config.get("jwt", "issuer").or(None),
        })
    }

    pub fn server_port_mut(&mut self) -> &mut u16 {
        &mut self.server_port
    }

    pub fn server_password_mut(&mut self) -> &mut Option<String> {
        &mut self.server_password
    }

    pub fn jwt_secret_mut(&mut self) -> &mut Option<String> {
        &mut self.jwt_secret
    }

    pub fn jwt_issuer_mut(&mut self) -> &mut Option<String> {
        &mut self.jwt_issuer
    }
}
