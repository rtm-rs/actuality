//! TLS Options
//!
//! | Environment                       | Server Flag           | Description                                                  |
//! |-----------------------------------|-----------------------|--------------------------------------------------------------|
//! | `RTM_JS_SERVER_ROUTES`            | `--routes`            | Comma-separated list of cluster URLs to solicit and connect. |
//! | `RTM_JS_SERVER_CLUSTER`           | `--cluster`           | Cluster URL for clustering requests.                         |
//! | `RTM_JS_SERVER_NO_ADVERTISE`      | `--no_advertise`      | Do not advertise known cluster information to clients.       |
//! | `RTM_JS_SERVER_CLUSTER_ADVERTISE` | `--cluster_advertise` | Cluster URL to advertise to other servers.                   |
//! | `RTM_JS_SERVER_CONNECT_RETRIES`   | `--connect_retries`   | For implicit routes, number of connect retries.              |
