//! | Item          | Description |
//! |---------------|-------------|
//! | Name          | A name for the Stream that may not have spaces, tabs, period (.), greater than (>) or asterisk (*). See |
//! | Storage       | The type of storage backend, File and Memory |
//! | Subjects      | A list of subjects to consume, supports wildcards |
//! | Replicas      | How many replicas to keep for each message in a clustered JetStream, maximum 5 |
//! | MaxAge        | Maximum age of any message in the Stream, expressed in nanoseconds. |
//! | MaxBytes      | How many bytes the Stream may contain. Adheres to Discard Policy, removing oldest or refusing new messages if the Stream exceeds this size |
//! | MaxMsgs       | How many messages may be in a Stream. Adheres to Discard Policy, removing oldest or refusing new messages if the Stream exceeds this number of messages |
//! | MaxMsgSize    | The largest message that will be accepted by the Stream |
//! | MaxConsumers  | How many Consumers can be defined for a given Stream, -1 for unlimited |
//! | NoAck         | Disables acknowledging messages that are received by the Stream |
//! | Retention     | How message retention is considered, LimitsPolicy (default), InterestPolicy or WorkQueuePolicy |
//! | Discard       | When a Stream reaches it's limits either, DiscardNew refuses new messages while DiscardOld (default) deletes old messages |
//! | Duplicates    | The window within which to track duplicate messages, expressed in nanoseconds. |

// https://docs.nats.io/nats-concepts/jetstream/streams
// Example: Rust stream, producer, consumer
// https://gist.github.com/wallyqs/05516d550b756e8b453394be0e9cbf24


// StreamConfig

/// `Stream` determines the properties for a stream.
/// There are sensible defaults for most. If no subjects are
/// given the name will be used as the only subject.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Stream {
    /// A name for the Stream. Must not have spaces, tabs or period `.` characters
    name: String,
    /// How large the Stream may become in total bytes before the configured discard policy kicks in
    max_bytes: i64,
    /// How large the Stream may become in total messages before the configured discard policy kicks in
    max_msgs: i64,
    /// Maximum amount of messages to keep per subject
    max_msgs_per_subject: i64,
    /// When a Stream has reached its configured `max_bytes` or `max_msgs`, this policy kicks in.
    /// `DiscardPolicy::New` refuses new messages or `DiscardPolicy::Old` (default) deletes old messages to make space
    discard: DiscardPolicy,
    /// Which NATS subjects to populate this stream with. Supports wildcards. Defaults to just the
    /// configured stream `name`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    subjects: Vec<String>,
    /// How message retention is considered, `Limits` (default), `Interest` or `WorkQueue`
    retention: RetentionPolicy,
    /// How many Consumers can be defined for a given Stream, -1 for unlimited
    max_consumers: i32,
    /// Maximum age of any message in the stream, expressed in nanoseconds
    #[serde(with = "serde_nanos")]
    max_age: Duration,
    /// The largest message that will be accepted by the Stream
    #[serde(default, skip_serializing_if = "is_default")]
    max_msg_size: i32,
    /// The type of storage backend, `File` (default) and `Memory`
    storage: StorageType,
    /// How many replicas to keep for each message in a clustered JetStream, maximum 5
    num_replicas: usize,
    /// Disables acknowledging messages that are received by the Stream
    #[serde(default, skip_serializing_if = "is_default")]
    no_ack: bool,
    /// The window within which to track duplicate messages.
    #[serde(default, skip_serializing_if = "is_default")]
    duplicate_window: i64,
    /// The owner of the template associated with this stream.
    #[serde(default, skip_serializing_if = "is_default")]
    template_owner: String,
    /// Indicates the stream is sealed and cannot be modified in any way
    #[serde(default, skip_serializing_if = "is_default")]
    sealed: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    /// A short description of the purpose of this stream.
    description: Option<String>,
    #[serde(
        default,
        rename = "allow_rollup_hdrs",
        skip_serializing_if = "is_default"
    )]
    /// Indicates if rollups will be allowed or not.
    allow_rollup: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    /// Indicates deletes will be denied or not.
    deny_delete: bool,
    /// Indicates if purges will be denied or not.
    #[serde(default, skip_serializing_if = "is_default")]
    deny_purge: bool,
}
