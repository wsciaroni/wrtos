# wrtos

A deterministic, performant, and simple Real-Time Operating System (RTOS) written in Rust.

`wrtos` utilizes a **Time-Triggered / Cyclic Executive** model. It does not support interrupts; instead, all operations are scheduled statically to guarantee absolute determinism. System execution is governed by a single hardware timer that ticks at a fixed interval.

---

## Architecture Overview

The repository is structured as a Cargo workspace containing the following crates:

*   **[`wrtos-core`](wrtos/wrtos-core)**: The `no_std`-compatible core RTOS runtime. It contains the `Timer` trait abstraction and the `CyclicExecutive` loop, which handles frame scheduling, task invocation, frame progression, and frame overrun detection.
*   **[`wrtos-codegen`](file:///c:/Users/wscia/dev/wrtos/wrtos-codegen)**: A compile-time schedule generator. It reads a declarative schedule configuration (`schedule.toml`), validates configuration correctness, and generates a static schedule table (`&[&[fn()]]`) in Rust.
*   **[`wrtos-wcet`](file:///c:/Users/wscia/dev/wrtos/wrtos-wcet)**: A Worst-Case Execution Time (WCET) profiling and verification library. It dynamically measures task execution times and verifies whether the sum of task WCETs in each minor cycle frame fits within the frame's microsecond budget.
*   **[`examples/simulation`](file:///c:/Users/wscia/dev/wrtos/examples/simulation)**: A host-based simulation example. It shows compile-time schedule generation, runs the WCET analysis phase, validates the schedule's feasibility, and executes the RTOS using a simulated high-resolution host timer.

---

## Configuration (`schedule.toml`)

The system schedule is configured declaratively in a `schedule.toml` file at the root of your application:

```toml
[system]
tick_rate_hz = 10         # 10 Hz tick rate = 100 ms minor cycle (frame) duration
major_cycle_frames = 5    # Major cycle is 5 frames = 500 ms repeating sequence

# Slotted tasks
[[tasks]]
name = "sensor_read"
func = "crate::tasks::sensor_read"
frames = [0, 2, 4]        # Runs on frames 0, 2, and 4

[[tasks]]
name = "control_loop"
func = "crate::tasks::control_loop"
frames = [1, 3]           # Runs on frames 1 and 3

[[tasks]]
name = "telemetry_send"
func = "crate::tasks::telemetry_send"
frames = [4]              # Runs on frame 4
```

During build time, a `build.rs` script calls `wrtos-codegen` to translate this file into a static array module included in the application:

```rust
pub const TICK_RATE_HZ: u32 = 10;
pub const MAJOR_CYCLE_FRAMES: usize = 5;

pub const SCHEDULE: &[&[fn()]] = &[
    &[crate::tasks::sensor_read],                    // Frame 0
    &[crate::tasks::control_loop],                   // Frame 1
    &[crate::tasks::sensor_read],                    // Frame 2
    &[crate::tasks::control_loop],                   // Frame 3
    &[crate::tasks::sensor_read, crate::tasks::telemetry_send], // Frame 4
];
```

---

## Worst-Case Execution Time (WCET) Analysis

To ensure absolute determinism, `wrtos-wcet` validates that the tasks mapped to any given frame do not exceed the frame duration budget:

\[
\text{Total WCET}_k = \sum_{T \in \text{Frame } k} \text{WCET}(T) \le \text{Frame Duration}
\]

If any frame violates this inequality, a budget overrun is detected. When running on real target hardware, frame overruns are treated as critical system faults that panic or trigger system resets to ensure safety.

---

## Getting Started

### Prerequisites

Make sure you have Rust installed (supports Rust 2021 edition).

### Running the Simulation

Run the simulated RTOS demonstrating compile-time codegen, WCET profiling, and execution:

```bash
cargo run --manifest-path examples/simulation/Cargo.toml
```

### Running the Tests

All crates are written using **Test-Driven Development (TDD)** and verified to maintain **100% decision coverage**:

```bash
cargo test --workspace
```
