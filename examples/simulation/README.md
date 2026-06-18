# wrtos Simulation Example

This directory contains a host-based simulation of the `wrtos` Cyclic Executive real-time operating system. 

It runs on your local machine using standard threads and high-precision sleeps to simulate real-time periodic ticks.

---

## Running the Simulation

Execute the following command from the workspace root or the simulation directory:

```bash
cargo run --manifest-path examples/simulation/Cargo.toml
```

---

## What Happens When Run?

The execution flows through three distinct phases:

### Phase 1: Dynamic WCET Profiling
The simulation starts by profiling each of the task implementations defined in [`src/tasks.rs`](file:///c:/Users/wscia/dev/wrtos/examples/simulation/src/tasks.rs) (without printing logs, to keep measurements clean).
*   It runs each task 10 times.
*   It measures the minimum, maximum (Worst-Case Execution Time - WCET), and average runtime using the high-resolution system clock.

### Phase 2: Schedule Feasibility Analysis
The profiler feeds the measured task WCET values and the static schedule parsed from [`schedule.toml`](file:///c:/Users/wscia/dev/wrtos/examples/simulation/schedule.toml) into the `wrtos-wcet` analyzer.
*   It calculates the frame budget in milliseconds (100 ms for a 10 Hz tick rate).
*   It sums the task WCETs for each frame (e.g., Frame 4 runs both `sensor_read` and `telemetry_send`, so its total WCET is the sum of both).
*   It verifies that no frame exceeds the budget limit. If the schedule is feasible, it proceeds; otherwise, it warns of an overrun.

### Phase 3: RTOS Execution Loop
It boots the cyclic executive and runs the time-triggered scheduler for **2 major cycles (10 steps)**.
*   **Frame 0**: Invokes `sensor_read` (~5 ms).
*   **Frame 1**: Invokes `control_loop` (~10 ms).
*   **Frame 2**: Invokes `sensor_read`.
*   **Frame 3**: Invokes `control_loop`.
*   **Frame 4**: Invokes `sensor_read` and `telemetry_send` (~20 ms total).
*   **Frame 5**: Wraps back around to Frame 0.

If a task were to run longer than its allocated frame, the timer would trigger a frame overrun error and panic.

---

## Modifying the Schedule

You can customize the execution loop:
1.  Open [`schedule.toml`](file:///c:/Users/wscia/dev/wrtos/examples/simulation/schedule.toml).
2.  Modify the tick rate, major cycle frames, or task slot allocations. For example, add a task function path or change which frames it runs on.
3.  Run `cargo run --manifest-path examples/simulation/Cargo.toml` again.
4.  Cargo's build system automatically detects changes to the TOML configuration, re-runs `build.rs` to generate the new `schedule.rs` static array, and executes the updated cyclic executive.
