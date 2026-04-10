use std::{sync::OnceLock, time::Duration};

use ferroid::{define_snowflake_id, generator::AtomicSnowflakeGenerator, time::MonotonicClock};

define_snowflake_id!(
    ArchtiSnowflakeId, u64,
    reserved: 1,
    timestamp: 41,
    machine_id: 10,
    sequence: 12
);

const ARCHTI_EPOCH: Duration = Duration::from_secs(1_704_067_200); // 2024-01-01 UTC

static GENERATOR: OnceLock<AtomicSnowflakeGenerator<ArchtiSnowflakeId, MonotonicClock<1>>> =
    OnceLock::new();

pub fn snowflake() -> i64 {
    let generator = GENERATOR.get_or_init(|| {
        let machine_id = 1_u64; // ideal: vir de env/config
        AtomicSnowflakeGenerator::new(machine_id, MonotonicClock::<1>::with_epoch(ARCHTI_EPOCH))
    });

    let raw = generator.next_id(|_| std::thread::yield_now()).to_raw();
    i64::try_from(raw).expect("snowflake must fit in i64")
}
