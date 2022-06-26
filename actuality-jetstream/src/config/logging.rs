//! Logging Options
//!
//! | Environment                   | Server Flag             | Description                                                    |
//! |-------------------------------|-------------------------|----------------------------------------------------------------|
//! | `RTM_JS_SERVER_LOG`           | `-l`, `--log`           | File to redirect log output.                                   |
//! | `RTM_JS_SERVER_LOGTIME`       | `-T`, `--logtime`       | Specify `-T=false` to disable timestamping log entries.        |
//! | `RTM_JS_SERVER_SYSLOG`        | `-s`, `--syslog`        | Log to syslog or windows event log.                            |
//! | `RTM_JS_SERVER_REMOTE_SYSLOG` | `-r`, `--remote_syslog` | The syslog server address, like `udp://localhost:514`.         |
//! | `RTM_JS_SERVER_V`             | `--debug`               | Enable debugging output.                                       |
//! | `RTM_JS_SERVER_VV`            | `--trace`               | Enable protocol trace log messages.                            |
//! | `RTM_JS_SERVER_VVV`           | `-VV`                   | Verbose trace (traces system account as well).                 |
//! | `RTM_JS_SERVER_VVVV`          | `-DV`                   | Enable both debug and protocol trace messages.                 |
//! | `RTM_JS_SERVER_VVVVV`         | `-DVV`                  | Debug and verbose trace (traces system account as well).       |
//! | `RTM_JS_SERVER_MAX_MSG_LEN`   | `--max_traced_msg_len`  | Maximum printable length for traced messages. 0 for unlimited. |
