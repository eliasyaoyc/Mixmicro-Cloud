pub struct Config {
    ping_timeout: usize,
    join_timeout: usize,
    join_retry_attempts: usize,
    join_retry_delay: usize,
    max_pings_from_another_master: usize,
    send_leave_request: bool,
    master_election_wait_for_joins_timeout: usize,
    master_election_ignore_non_master_pings: bool,
    publish_max_pending_cluster_status: usize,
}

pub struct Builder {
    ping_timeout: usize,
    join_timeout: usize,
    join_retry_attempts: usize,
    join_retry_delay: usize,
    max_pings_from_another_master: usize,
    send_leave_request: bool,
    master_election_wait_for_joins_timeout: usize,
    master_election_ignore_non_master_pings: bool,
    publish_max_pending_cluster_status: usize,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            ping_timeout: 3, // s
            join_timeout: 60000, // millis
            join_retry_attempts: 3,
            join_retry_delay: 100, // millis
            max_pings_from_another_master: 3,
            send_leave_request: true,
            master_election_wait_for_joins_timeout: 30000, // millis
            master_election_ignore_non_master_pings: false,
            publish_max_pending_cluster_status: 25,
        }
    }
}

impl Builder {
    pub fn builder() -> Builder {
        Default::default()
    }

    pub fn ping_timeout(mut self, ping_timeout: usize) -> Builder {
        self.ping_timeout = ping_timeout;
        self
    }

    pub fn join_timeout(mut self, join_timeout: usize) -> Builder {
        self.join_timeout = join_timeout;
        self
    }

    pub fn join_retry_attempts(mut self, join_retry_attempts: usize) -> Builder {
        self.join_retry_attempts = join_retry_attempts;
        self
    }

    pub fn join_retry_delay(mut self, join_retry_delay:usize) -> Builder {
        self.join_retry_delay = join_retry_delay;
        self
    }

    pub fn max_pings_from_another_master(mut self, max_pings_from_another_master: usize) -> Builder {
        self.max_pings_from_another_master = max_pings_from_another_master;
        self
    }

    pub fn send_leave_request(mut self, send_leave_request: bool) -> Builder {
        self.send_leave_request = send_leave_request;
        self
    }

    pub fn master_election_wait_for_joins_timeout(mut self, master_election_wait_for_joins_timeout: usize) -> Builder{
        self.master_election_wait_for_joins_timeout = master_election_wait_for_joins_timeout;
        self
    }

    pub fn master_election_ignore_non_master_pings(mut self, master_election_ignore_non_master_pings: bool) -> Builder{
        self.master_election_ignore_non_master_pings = master_election_ignore_non_master_pings;
        self
    }

    pub fn publish_max_pending_cluster_status(mut self, publish_max_pending_cluster_status: usize) -> Builder{
        self.publish_max_pending_cluster_status = publish_max_pending_cluster_status;
        self
    }

    pub fn build(self) -> Config {
        Config {
            ping_timeout: self.ping_timeout,
            join_timeout: self.join_timeout,
            join_retry_attempts: self.join_retry_attempts,
            join_retry_delay: self.join_retry_delay,
            max_pings_from_another_master: self.max_pings_from_another_master,
            send_leave_request: self.send_leave_request,
            master_election_wait_for_joins_timeout: self.master_election_wait_for_joins_timeout,
            master_election_ignore_non_master_pings: self.master_election_ignore_non_master_pings,
            publish_max_pending_cluster_status: self.publish_max_pending_cluster_status,
        }
    }
}