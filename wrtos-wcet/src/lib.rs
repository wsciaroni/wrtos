use std::collections::HashMap;
use std::time::{Duration, Instant};
use wrtos_codegen::ScheduleConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskStats {
    pub min: Duration,
    pub max: Duration,
    pub avg: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameReport {
    pub frame_index: usize,
    pub total_wcet: Duration,
    pub budget: Duration,
    pub remaining: Duration,
    pub overrun: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisReport {
    pub frame_reports: Vec<FrameReport>,
    pub has_overruns: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnalysisError {
    MissingTaskWcet(String),
    ZeroTickRate,
}

pub fn profile_task(task: fn(), iterations: usize) -> TaskStats {
    if iterations == 0 {
        return TaskStats {
            min: Duration::from_secs(0),
            max: Duration::from_secs(0),
            avg: Duration::from_secs(0),
        };
    }

    let mut min = Duration::MAX;
    let mut max = Duration::ZERO;
    let mut total = Duration::ZERO;

    for _ in 0..iterations {
        let start = Instant::now();
        task();
        let elapsed = start.elapsed();

        if elapsed < min {
            min = elapsed;
        }
        if elapsed > max {
            max = elapsed;
        }
        total += elapsed;
    }

    let avg = total / (iterations as u32);

    TaskStats { min, max, avg }
}

pub fn analyze_schedule(
    config: &ScheduleConfig,
    task_wcets: &HashMap<String, Duration>,
) -> Result<AnalysisReport, AnalysisError> {
    if config.system.tick_rate_hz == 0 {
        return Err(AnalysisError::ZeroTickRate);
    }

    let budget = Duration::from_micros(1_000_000 / config.system.tick_rate_hz as u64);

    let mut frame_reports = Vec::new();
    let mut has_overruns = false;

    // Optimization: Replace O(F * T * F_T) loop over frames and tasks
    // with O(T * F_T) loop by aggregating WCETs into frames.
    let mut frame_wcets = vec![Duration::ZERO; config.system.major_cycle_frames];

    for task in &config.tasks {
        let &wcet = task_wcets.get(&task.name).ok_or_else(|| {
            AnalysisError::MissingTaskWcet(task.name.clone())
        })?;
        for &frame in &task.frames {
            if frame < config.system.major_cycle_frames {
                frame_wcets[frame] += wcet;
            }
        }
    }

    for (i, total_wcet) in frame_wcets.into_iter().enumerate() {
        let overrun = total_wcet > budget;
        if overrun {
            has_overruns = true;
        }

        let remaining = if total_wcet <= budget {
            budget - total_wcet
        } else {
            Duration::ZERO
        };

        frame_reports.push(FrameReport {
            frame_index: i,
            total_wcet,
            budget,
            remaining,
            overrun,
        });
    }

    Ok(AnalysisReport {
        frame_reports,
        has_overruns,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wrtos_codegen::{ScheduleConfig, SystemConfig, TaskConfig};

    // Helper to create a dummy ScheduleConfig
    fn create_test_config(tick_rate_hz: u32, major_cycle_frames: usize) -> ScheduleConfig {
        ScheduleConfig {
            system: SystemConfig {
                tick_rate_hz,
                major_cycle_frames,
            },
            tasks: vec![
                TaskConfig {
                    name: "t1".to_string(),
                    func: "t1_func".to_string(),
                    frames: vec![0, 2],
                },
                TaskConfig {
                    name: "t2".to_string(),
                    func: "t2_func".to_string(),
                    frames: vec![1],
                },
            ],
        }
    }

    #[test]
    fn test_analyze_schedule_success() {
        let config = create_test_config(1000, 3); // 1 ms (1000 us) frame budget
        let mut task_wcets = HashMap::new();
        task_wcets.insert("t1".to_string(), Duration::from_micros(300));
        task_wcets.insert("t2".to_string(), Duration::from_micros(600));

        let res = analyze_schedule(&config, &task_wcets);
        assert!(res.is_ok());
        let report = res.unwrap();
        assert_eq!(report.has_overruns, false);

        // Frame 0 budget: 1ms. Contains t1 (300us). Remaining: 700us.
        assert_eq!(report.frame_reports[0].total_wcet, Duration::from_micros(300));
        assert_eq!(report.frame_reports[0].remaining, Duration::from_micros(700));
        assert_eq!(report.frame_reports[0].overrun, false);

        // Frame 1 budget: 1ms. Contains t2 (600us). Remaining: 400us.
        assert_eq!(report.frame_reports[1].total_wcet, Duration::from_micros(600));
        assert_eq!(report.frame_reports[1].remaining, Duration::from_micros(400));
        assert_eq!(report.frame_reports[1].overrun, false);

        // Frame 2 budget: 1ms. Contains t1 (300us). Remaining: 700us.
        assert_eq!(report.frame_reports[2].total_wcet, Duration::from_micros(300));
        assert_eq!(report.frame_reports[2].remaining, Duration::from_micros(700));
        assert_eq!(report.frame_reports[2].overrun, false);
    }

    #[test]
    fn test_analyze_schedule_overrun() {
        let config = create_test_config(1000, 3);
        let mut task_wcets = HashMap::new();
        // t2 WCET is 1200us, which exceeds 1000us budget
        task_wcets.insert("t1".to_string(), Duration::from_micros(300));
        task_wcets.insert("t2".to_string(), Duration::from_micros(1200));

        let res = analyze_schedule(&config, &task_wcets);
        assert!(res.is_ok());
        let report = res.unwrap();
        assert_eq!(report.has_overruns, true);
        assert_eq!(report.frame_reports[1].overrun, true);
        assert_eq!(report.frame_reports[1].total_wcet, Duration::from_micros(1200));
    }

    #[test]
    fn test_analyze_schedule_missing_task_wcet() {
        let config = create_test_config(1000, 3);
        let mut task_wcets = HashMap::new();
        task_wcets.insert("t1".to_string(), Duration::from_micros(300));
        // t2 WCET is missing!

        let res = analyze_schedule(&config, &task_wcets);
        assert_eq!(res, Err(AnalysisError::MissingTaskWcet("t2".to_string())));
    }

    #[test]
    fn test_analyze_schedule_zero_tick_rate() {
        let config = create_test_config(0, 3);
        let mut task_wcets = HashMap::new();
        task_wcets.insert("t1".to_string(), Duration::from_micros(300));
        task_wcets.insert("t2".to_string(), Duration::from_micros(300));

        let res = analyze_schedule(&config, &task_wcets);
        assert_eq!(res, Err(AnalysisError::ZeroTickRate));
    }

    // A dummy workload for profiling
    fn dummy_task() {
        // Spin for a tiny bit
        let start = Instant::now();
        while start.elapsed() < Duration::from_micros(10) {}
    }

    #[test]
    fn test_profile_task_execution() {
        let stats = profile_task(dummy_task, 10);
        assert!(stats.min > Duration::from_secs(0));
        assert!(stats.max >= stats.min);
        assert!(stats.avg >= stats.min);
        assert!(stats.avg <= stats.max);
    }

    #[test]
    fn test_profile_task_zero_iterations() {
        let stats = profile_task(dummy_task, 0);
        assert_eq!(stats.min, Duration::ZERO);
        assert_eq!(stats.max, Duration::ZERO);
        assert_eq!(stats.avg, Duration::ZERO);
    }
}
