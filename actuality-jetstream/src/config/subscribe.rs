// SubscribeOptions
// PullSubscribeOptions

/// Subscription configuration
#[derive(Debug, Default, Clone)]
struct Subscribe {
    // For consumer binding:
    bind_only: bool,
    consumer_name: Option<String>,
    stream_name: Option<String>,

    // For consumer configuration:
    ack_policy: Option<AckPolicy>,
    ack_wait: Option<Duration>,
    deliver_policy: Option<DeliverPolicy>,
    deliver_subject: Option<String>,
    description: Option<String>,
    durable_name: Option<String>,
    flow_control: Option<bool>,
    headers_only: Option<bool>,
    idle_heartbeat: Option<Duration>,
    max_ack_pending: Option<i64>,
    max_deliver: Option<i64>,
    max_waiting: Option<i64>,
    opt_start_seq: Option<u64>,
    opt_start_time: Option<DateTime>,
    ordered: bool,
    rate_limit: Option<u64>,
    replay_policy: Option<ReplayPolicy>,
    sample_frequency: Option<u8>,
}

/// Options to configure Pull Subscription
#[derive(Debug, Default, Clone)]
struct PullSubscribe {
    bind_only: bool,
    consumer_config: Option<ConsumerConfig>,
    durable_name: Option<String>,
    stream_name: Option<String>,
}