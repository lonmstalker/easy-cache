mod server {}

pub mod uni {
    use std::fmt::{Debug, Formatter};
    use bytes::Bytes;
    use uuid::Uuid;

    /**
    Value request with parts of cache
     */
    #[derive(Default, Debug)]
    pub struct DataRequest {
        rq_id: Uuid,
        data: MessagePart,
    }

    /**
    Part of cache message
     */
    #[derive(Default, Debug)]
    pub enum MessagePart {
        Hello { key: String, parts: usize, ttl: Option<i64> },
        Data { part_id: usize, value: Bytes },
        End,
    }
}