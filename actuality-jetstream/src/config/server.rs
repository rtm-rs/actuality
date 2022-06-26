//! Server Options
//!
//! | Environment                      | Server Flag               | Description  |
//! |----------------------------------|---------------------------|--------------|
//! | `RTM_JS_SERVER_ADDR`             | `-a`,  `--addr`           | Host address to bind to (default: 0.0.0.0 - all interfaces). |
//! | `RTM_JS_SERVER_PORT`             | `-p`,  `--port`           | NATS client port (default: 4222).   |
//! | `RTM_JS_SERVER_PID`              | `-P`,  `--pid`            | File to store the process ID (PID). |
//! | `RTM_JS_SERVER_HTTP_PORT`        | `-m`,  `--http_port`      | HTTP port for monitoring dashboard (exclusive of --https_port). |
//! | `RTM_JS_SERVER_HTTPS_PORT`       | `-ms`, `--https_port`     | HTTPS port monitoring for monitoring dashboard (exclusive of --http_port). |
//! | `RTM_JS_SERVER_CONFIG`           | `-c`,  `--config`         | Path to NATS server configuration file.        |
//! | `RTM_JS_SERVER_SIGNAL`           | `-sl`, `--signal`         | Send a signal to nats-server process.          |
//! | `RTM_JS_SERVER_CLIENT_ADVERTISE` | `--client_advertise`      | Client HostPort to advertise to other servers. |

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    #[serde(default="default_addr")]
    addr: String,
    client_advertise: Option<String>,
    config: Option<String>,
    http_port: Option<String>,
    https_port: Option<String>,
    pid: Option<String>,
    #[serde(default="default_port")]
    port: usize,
}

enum Field {
    Address(String),
    ClientAdvertise(String),
    Config(String),
    HttpPort(String),
    HttpsPort(String),
    Pid(String),
    Port(usize),
}

impl Default for Server {
    fn default() -> Self {
        Self {
            /// Host address to bind to (default: 0.0.0.0 - all interfaces).
            addr: default_addr(),
            /// Client HostPort to advertise to other servers (default: None).
            client_advertise: None,
            /// Path to NATS server configuration file (default: None).
            config: None,
            /// HTTP port for monitoring dashboard exclusive of RTM_JS_SERVER_HTTPS_PORT (default: None).
            http_port: None,
            /// HTTPS port for monitoring dashboard exclusive of RTM_JS_SERVER_HTTP_PORT (default: None).
            https_port: None,
            /// File to store the process ID (default: None).
            pid: None,
            /// NATS client port (default: 4222).
            port: default_port(),
        }
    }
}

fn default_port() -> usize {
    4222
}

fn default_addr() -> String {
    "0.0.0.0".to_string()
}

impl Server {
    fn new() -> Self {
        match envy::prefixed("RTM_JS_SERVER_").from_env::<Server>() {
            Ok(config) => config,
            Err(_error) => Self { ..Default::default() },
        }
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Notice we're using `Field` instead of `Box<dyn ...>`
        let mut fields: Vec<(&str, Field)> = vec![("addr", Field::Address(self.addr.clone()))];

        if let Some(client_advertise) = &self.client_advertise {
            let value = (*client_advertise).clone();
            fields.push(("client_advertise", Field::ClientAdvertise(value)));
        }
        if let Some(config) = &self.config {
            let value = (*config).clone();
            fields.push(("config", Field::Config(value)));
        }
        if let Some(http_port) = &self.http_port {
            let value = (*http_port).clone();
            fields.push(("http_port", Field::HttpPort(value)));
        }
        if let Some(https_port) = &self.https_port {
            let value = (*https_port).clone();
            fields.push(("https_port", Field::HttpsPort(value)));
        }
        if let Some(pid) = &self.pid {
            let value = (*pid).clone();
            fields.push(("pid", Field::Pid(value)));
        }

        fields.push(("port", Field::Port(self.port)));

        for (name, field) in &fields {
            // Manually dispatching based on the field variant we have.
            match field {
                Field::Address(value) => make_flag(fmt, *name, value)?,
                Field::ClientAdvertise(value) => make_flag(fmt, *name, value)?,
                Field::Config(value) => make_flag(fmt, *name, value)?,
                Field::HttpPort(value) => make_flag(fmt, *name, value)?,
                Field::HttpsPort(value) => make_flag(fmt, *name, value)?,
                Field::Pid(value) => make_flag(fmt, *name, value)?,
                Field::Port(value) => make_flag(fmt, *name, &value.to_string())?,
            };
        }

        Ok(())
    }
}

fn make_flag(f: &mut std::fmt::Formatter<'_>, name: &str, value: &str) -> std::fmt::Result {
    f.write_str(" --")?;
    f.write_str(name)?;
    f.write_str(" ")?;
    f.write_str(value)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn config_server_default() {
        // remove variables - tests are run in the same environment
        std::env::remove_var("RTM_JS_SERVER_ADDR");
        std::env::remove_var("RTM_JS_SERVER_PORT");
        let server = match envy::prefixed("RTM_JS_SERVER_").from_env::<Server>() {
            Ok(config) => config,
            Err(_error) => Server { addr: "1.1.1.1".to_string(), port: 4444, ..Default::default() },
        };
        assert_eq!(server.addr, "0.0.0.0".to_string());
        assert_eq!(server.client_advertise, None);
        assert_eq!(server.config, None);
        assert_eq!(server.http_port, None);
        assert_eq!(server.https_port, None);
        assert_eq!(server.client_advertise, None);
        assert_eq!(server.pid, None);
        assert_eq!(server.port, 4222);
    }

    #[test]
    fn config_server() {
        std::env::set_var("RTM_JS_SERVER_ADDR", "127.0.0.1");
        std::env::set_var("RTM_JS_SERVER_PORT", "4223");
        let server = match envy::prefixed("RTM_JS_SERVER_").from_env::<Server>() {
            Ok(config) => config,
            Err(_error) => Server { addr: "1.1.1.1".to_string(), port: 4444, ..Default::default() },
        };
        assert_eq!(server.addr, "127.0.0.1".to_string());
        assert_eq!(server.client_advertise, None);
        assert_eq!(server.config, None);
        assert_eq!(server.http_port, None);
        assert_eq!(server.https_port, None);
        assert_eq!(server.client_advertise, None);
        assert_eq!(server.pid, None);
        assert_eq!(server.port, 4223);
    }

    #[test]
    fn config_server_flags() {
        std::env::set_var("RTM_JS_SERVER_ADDR", "127.0.0.1");
        std::env::set_var("RTM_JS_SERVER_PORT", "4223");
        let server = Server::new();
        assert_eq!(server.to_string(), " --addr 127.0.0.1 --port 4223")
    }

    #[test]
    fn config_server_flags_default() {
        // remove variables - tests are run in the same environment
        std::env::remove_var("RTM_JS_SERVER_ADDR");
        std::env::remove_var("RTM_JS_SERVER_PORT");

        let server = Server::new();
        assert_eq!(server.to_string(), " --addr 0.0.0.0 --port 4222")
    }
}
