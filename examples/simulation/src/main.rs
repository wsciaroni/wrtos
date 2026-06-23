use std::collections::HashMap;
use std::time::{Duration, Instant};

pub mod tasks;

// Include the compile-time generated schedule configuration
include!(concat!(env!("OUT_DIR"), "/schedule.rs"));

// Host simulated timer implementing wrtos_core::Timer
struct SimulatedTimer {
    start_time: Instant,
    tick_interval: Duration,
    tick_count: u64,
    overrun_occurred: bool,
}

impl SimulatedTimer {
    fn new(tick_rate_hz: u32) -> Self {
        Self {
            start_time: Instant::now(),
            tick_interval: Duration::from_micros(1_000_000 / tick_rate_hz as u64),
            tick_count: 0,
            overrun_occurred: false,
        }
    }
}

impl wrtos_core::Timer for SimulatedTimer {
    fn wait_next_tick(&mut self) {
        self.tick_count += 1;
        let target_elapsed = self.tick_interval * (self.tick_count as u32);
        let actual_elapsed = self.start_time.elapsed();

        if actual_elapsed > target_elapsed {
            self.overrun_occurred = true;
        } else {
            let sleep_dur = target_elapsed - actual_elapsed;
            std::thread::sleep(sleep_dur);
        }
    }

    fn has_overrun(&self) -> bool {
        self.overrun_occurred
    }
}

fn main() {
    println!("=== Time-Triggered Cyclic Executive RTOS Simulation ===");
    println!("System Tick Rate: {} Hz (Frame duration: {} ms)", TICK_RATE_HZ, 1000 / TICK_RATE_HZ);
    println!("Major Cycle: {} frames ({} ms)", MAJOR_CYCLE_FRAMES, (1000 / TICK_RATE_HZ) * MAJOR_CYCLE_FRAMES as u32);
    println!("------------------------------------------------------");

    // 1. WCET Profiling Phase
    println!("Phase 1: Running WCET Profiler on tasks...");
    let iterations = 10; // keep it short for quick running

    // Profile the task implementations (without stdout printing to prevent clutter)
    let stats_sensor = wrtos_wcet::profile_task(tasks::sensor_read_impl, iterations);
    let stats_control = wrtos_wcet::profile_task(tasks::control_loop_impl, iterations);
    let stats_telemetry = wrtos_wcet::profile_task(tasks::telemetry_send_impl, iterations);

    println!("Task Statistics (Min / WCET / Avg):");
    println!("  sensor_read:    {:?} / {:?} / {:?}", stats_sensor.min, stats_sensor.max, stats_sensor.avg);
    println!("  control_loop:   {:?} / {:?} / {:?}", stats_control.min, stats_control.max, stats_control.avg);
    println!("  telemetry_send: {:?} / {:?} / {:?}", stats_telemetry.min, stats_telemetry.max, stats_telemetry.avg);

    // 2. Schedule Feasibility Analysis Phase
    println!("\nPhase 2: Analyzing schedule feasibility...");
    
    // Load schedule.toml at compile-time to be independent of the current working directory
    let toml_content = include_str!("../schedule.toml");
    let config = wrtos_codegen::parse_config(toml_content)
        .expect("Failed to parse schedule.toml");

    let mut task_wcets = HashMap::new();
    task_wcets.insert("sensor_read".to_string(), stats_sensor.max);
    task_wcets.insert("control_loop".to_string(), stats_control.max);
    task_wcets.insert("telemetry_send".to_string(), stats_telemetry.max);

    match wrtos_wcet::analyze_schedule(&config, &task_wcets) {
        Ok(report) => {
            println!("Schedule Analysis Report:");
            for r in &report.frame_reports {
                println!(
                    "  Frame {}: Total WCET = {:?}, Budget = {:?}, Remaining = {:?}, Overrun = {}",
                    r.frame_index, r.total_wcet, r.budget, r.remaining, r.overrun
                );
            }
            if report.has_overruns {
                println!("\nWARNING: Schedule is NOT feasible! Frame budget overruns detected.");
            } else {
                println!("\nSUCCESS: Schedule is feasible! All frames fit within budget.");
            }
        }
        Err(e) => {
            println!("ERROR: Failed to analyze schedule: {:?}", e);
            std::process::exit(1);
        }
    }

    // 3. RTOS Execution Phase
    println!("\nPhase 3: Starting Cyclic Executive...");
    let mut timer = SimulatedTimer::new(TICK_RATE_HZ);
    let mut exec = wrtos_core::CyclicExecutive::new(SCHEDULE);

    // Run for 2 major cycles (10 frames)
    let total_steps = 10;
    for step_num in 0..total_steps {
        let frame = exec.current_frame();
        println!("\n--- [Time: {:?}] Frame {} (Step {}/{}) ---", Instant::now(), frame, step_num + 1, total_steps);
        
        if exec.step(&mut timer).is_err() {
            println!("CRITICAL ERROR: Frame overrun detected in frame {}!", frame);
            std::process::exit(1);
        }
    }

    println!("\nSimulation completed successfully after {} steps.", total_steps);
}
