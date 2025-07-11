use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start_primero_schedulers() -> Result<(), Box<dyn std::error::Error>> {
    
    // 🔧 Set up cron scheduler
    let scheduler = JobScheduler::new().await.unwrap();

    // 🕒 Add a cron job (every 5 minutes)
    let job = Job::new_async("* 5 * * * *", |_uuid, _lock| {
        Box::pin(async move {
            println!("🔄 Running cron task every 5 minutes...");
            // Your async logic here (e.g. DB cleanup, send email, etc.)
        })
    })
    .unwrap();

    scheduler.add(job).await.unwrap();
    scheduler.start().await.unwrap();

    Ok(())
}
