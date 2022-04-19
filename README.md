# Cron Next is a tool base on cron_clock and tokio, make it easy to use with cron jobs

# Examples
```rust
let expression = "* * * * * ? *";
let mut cron = CronNext::new(expression, chrono::Local)?;
while let Some(time) = cron.next().await {
  println!("time: {:?}, {:?}", time, chrono::Local::now());
}
println!("if not a forever loop job, will stop finally");
```