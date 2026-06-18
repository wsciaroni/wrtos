use std::time::Duration;

// Common implementation of tasks work
pub fn sensor_read_impl() {
    // Mock some work (e.g., spinning or sleeping)
    // 5ms sleep
    std::thread::sleep(Duration::from_millis(5));
}

pub fn control_loop_impl() {
    // 10ms sleep
    std::thread::sleep(Duration::from_millis(10));
}

pub fn telemetry_send_impl() {
    // 15ms sleep
    std::thread::sleep(Duration::from_millis(15));
}

// User-facing logging tasks
pub fn sensor_read() {
    println!("  [Task] sensor_read starting...");
    sensor_read_impl();
    println!("  [Task] sensor_read completed.");
}

pub fn control_loop() {
    println!("  [Task] control_loop starting...");
    control_loop_impl();
    println!("  [Task] control_loop completed.");
}

pub fn telemetry_send() {
    println!("  [Task] telemetry_send starting...");
    telemetry_send_impl();
    println!("  [Task] telemetry_send completed.");
}
