use metrickit::MetricManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MetricManager::shared();
    let log_handle = manager.make_log_handle("examples.05_signpost")?;

    let event_id = log_handle.make_signpost_id()?;
    log_handle.emit_event(event_id, "payload-received")?;

    let interval_id = log_handle.make_signpost_id()?;
    log_handle.interval_begin(interval_id, "payload-processing")?;
    log_handle.interval_end(interval_id, "payload-processing")?;

    let animation_id = log_handle.make_signpost_id()?;
    log_handle.animation_interval_begin(animation_id, "payload-animation")?;
    log_handle.interval_end(animation_id, "payload-animation")?;

    println!(
        "✅ emitted MetricKit signposts with category {}",
        log_handle.category()
    );
    Ok(())
}
