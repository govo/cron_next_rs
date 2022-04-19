//! # cron_next
//!
//! `cron_next` is a tool base on cron_clock and tokio, make it easy to use with cron jobs

use std::str::FromStr;

use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use cron_clock::Schedule;

/// cron jobs
///
/// # Examples
/// ```
/// let expression = "* * * * * ? *";
/// let mut cron = CronNext::new(expression, chrono::Local)?;
/// while let Some(time) = cron.next().await {
///    println!("time: {:?}, {:?}", time, chrono::Local::now());
/// }
/// println!("break");
/// ```
pub struct CronNext<Z: TimeZone> {
    pub schedule: Schedule,
    last: Option<DateTime<Z>>,
    duration: std::time::Duration,
    timezone: Z,
}
impl<Z> CronNext<Z>
where
    Z: TimeZone,
{
    /// create a cron
    /// # Examples
    /// ```
    /// let expression = "* * * * * ? *";
    /// let mut cron = CronNext::new(expression, chrono::Local)?;
    /// while let Some(time) = cron.next().await {
    ///    println!("time: {:?}, {:?}", time, chrono::Local::now());
    /// }
    /// println!("break");
    /// ```
    pub fn new(expression: &str, timezone: Z) -> Result<Self> {
        let duration = std::time::Duration::from_millis(1000);
        let schedule = Schedule::from_str(expression)?;
        let last = schedule.upcoming(timezone.clone()).next();
        Ok(Self {
            schedule,
            timezone,
            last,
            duration,
        })
    }
    /// the next job, ticker is 1 seconds
    /// # Examples
    /// ```
    /// let expression = "* * * * * ? *";
    /// let mut cron = CronNext::new(expression, chrono::Local)?;
    /// while let Some(time) = cron.next().await {
    ///    println!("time: {:?}, {:?}", time, chrono::Local::now());
    /// }
    /// println!("break");
    /// ```
    pub async fn next(&mut self) -> Option<DateTime<Z>> {
        let last_time = match self.last.take() {
            Some(l) => l,
            None => {
                let n = self.schedule.upcoming(self.timezone.clone()).next();

                match n {
                    Some(n) => n,
                    None => return None,
                }
            }
        };
        loop {
            let now = &self.timezone.from_utc_datetime(&Utc::now().naive_utc());
            if now.timestamp_nanos() >= last_time.timestamp_nanos() {
                return Some(last_time);
            } else {
                tokio::time::sleep(self.duration).await;
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[tokio::test]
    async fn test_cron() -> Result<()> {
        let expression = "* * * * * ? *";
        let mut cron = CronNext::new(expression, chrono::Local)?;
        while let Some(time) = cron.next().await {
            //cargo test test_cron -- --nocapture
            println!("time: {:?}, {:?}", time, chrono::Local::now());
        }
        println!("break");
        Ok(())
    }
}
