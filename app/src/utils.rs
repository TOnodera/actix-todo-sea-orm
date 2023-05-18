use chrono::{FixedOffset, Utc};

use crate::{configure::tz, types::Result};

pub fn now() -> Result<chrono::DateTime<FixedOffset>> {
    Ok(Utc::now().with_timezone(&tz()))
}
