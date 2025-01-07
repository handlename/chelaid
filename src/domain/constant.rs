pub const WORKER_ID_BITS: i32 = 10;
pub const SEQUENCE_BITS: i32 = 12;

// 2015-01-01T00:00:00Z in milliseconds
pub const TIMESTAMP_OFFSET: u64 = 1420070400_000;

pub const TIMESTAMP_MIN: u64 = TIMESTAMP_OFFSET;

pub const SEQUENCE_MIN: u32 = 0;
pub const SEQUENCE_MAX: u32 = (1 << SEQUENCE_BITS) - 1;