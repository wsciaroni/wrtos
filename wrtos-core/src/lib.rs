#![no_std]

#[cfg(test)]
extern crate std;

// The types we need to test.
// We declare them as stubs first so that the test code can compile, but the tests will fail.

pub trait Timer {
    fn wait_next_tick(&mut self);
    fn has_overrun(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
pub struct OverrunError;

pub struct CyclicExecutive {
    current_frame: usize,
    major_cycle_frames: usize,
    schedule: &'static [&'static [fn()]],
}

impl CyclicExecutive {
    pub fn new(schedule: &'static [&'static [fn()]]) -> Self {
        Self {
            current_frame: 0,
            major_cycle_frames: schedule.len(),
            schedule,
        }
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn step<T: Timer>(&mut self, timer: &mut T) -> Result<(), OverrunError> {
        if self.major_cycle_frames == 0 {
            return Ok(());
        }

        timer.wait_next_tick();

        let tasks = self.schedule[self.current_frame];
        for &task in tasks {
            task();
        }

        if timer.has_overrun() {
            return Err(OverrunError);
        }

        // Optimization: Avoid expensive modulo/division on MCUs without hardware support
        self.current_frame += 1;
        if self.current_frame >= self.major_cycle_frames {
            self.current_frame = 0;
        }
        Ok(())
    }

    pub fn run_loop<T: Timer>(&mut self, mut timer: T) -> ! {
        loop {
            if self.step(&mut timer).is_err() {
                panic!("Frame overrun");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    use std::string::String;
    use std::sync::Mutex;

    // A mock timer for testing
    struct MockTimer {
        tick_count: usize,
        overrun_value: bool,
    }

    impl Timer for MockTimer {
        fn wait_next_tick(&mut self) {
            self.tick_count += 1;
        }

        fn has_overrun(&self) -> bool {
            self.overrun_value
        }
    }

    // A thread-safe called task list
    static CALLED_TASKS: Mutex<Vec<String>> = Mutex::new(Vec::new());

    fn get_called_tasks() -> Vec<String> {
        CALLED_TASKS.lock().unwrap().clone()
    }

    fn clear_called_tasks() {
        CALLED_TASKS.lock().unwrap().clear();
    }

    fn record_task(name: &str) {
        CALLED_TASKS.lock().unwrap().push(String::from(name));
    }

    fn task_a() { record_task("A"); }
    fn task_b() { record_task("B"); }
    fn task_c() { record_task("C"); }

    #[test]
    fn test_normal_execution_flow() {
        clear_called_tasks();
        let schedule: &[&[fn()]] = &[
            &[task_a, task_b], // Frame 0
            &[],               // Frame 1
            &[task_c],         // Frame 2
        ];

        let mut exec = CyclicExecutive::new(schedule);
        let mut timer = MockTimer { tick_count: 0, overrun_value: false };

        assert_eq!(exec.current_frame(), 0);

        // Step 1: Frame 0
        let res = exec.step(&mut timer);
        assert_eq!(res, Ok(()));
        assert_eq!(timer.tick_count, 1);
        assert_eq!(exec.current_frame(), 1);
        assert_eq!(get_called_tasks(), ["A", "B"]);

        // Step 2: Frame 1
        let res = exec.step(&mut timer);
        assert_eq!(res, Ok(()));
        assert_eq!(timer.tick_count, 2);
        assert_eq!(exec.current_frame(), 2);
        assert_eq!(get_called_tasks(), ["A", "B"]); // No new tasks

        // Step 3: Frame 2
        let res = exec.step(&mut timer);
        assert_eq!(res, Ok(()));
        assert_eq!(timer.tick_count, 3);
        assert_eq!(exec.current_frame(), 0); // Wraps around to 0
        assert_eq!(get_called_tasks(), ["A", "B", "C"]);

        // Step 4: Frame 0 again
        let res = exec.step(&mut timer);
        assert_eq!(res, Ok(()));
        assert_eq!(timer.tick_count, 4);
        assert_eq!(exec.current_frame(), 1);
        assert_eq!(get_called_tasks(), ["A", "B", "C", "A", "B"]);
    }

    #[test]
    fn test_frame_overrun() {
        let schedule: &[&[fn()]] = &[&[task_a]];
        let mut exec = CyclicExecutive::new(schedule);
        let mut timer = MockTimer { tick_count: 0, overrun_value: true };

        let res = exec.step(&mut timer);
        assert_eq!(res, Err(OverrunError));
    }

    #[test]
    fn test_empty_schedule() {
        let schedule: &[&[fn()]] = &[];
        let mut exec = CyclicExecutive::new(schedule);
        let mut timer = MockTimer { tick_count: 0, overrun_value: false };

        let res = exec.step(&mut timer);
        assert_eq!(res, Ok(()));
        assert_eq!(timer.tick_count, 0); // Should not tick since major_cycle_frames is 0
    }

    #[test]
    fn test_run_loop_overrun_panic() {
        let schedule: &[&[fn()]] = &[&[task_a]];
        let mut exec = CyclicExecutive::new(schedule);
        let timer = MockTimer { tick_count: 0, overrun_value: true };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            exec.run_loop(timer);
        }));

        assert!(result.is_err());
    }
}
