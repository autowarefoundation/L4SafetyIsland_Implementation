# si-platform-host

## Purpose

`si-hal` on `std`, for unit tests, software-in-the-loop runs and the WP0 test harness.

## Contents

- `HostClock`: `si_hal::Clock` on `std::time`.
- `harness`: the WP0 test harness (WP0-WP3-SafetyFloor.md section 1.2):
  - `Publisher`: a periodic stream whose payload is a function of the frame
    counter and the send time, with faults injected for a time window:
    `Stop` (nothing sent), `Freeze` (the last intact frame resent with its
    counter and stamp) and `Corrupt` (a fresh frame that fails its
    end-to-end check). Each frame carries a counter, a stamp and an
    `intact` flag, which is what the receiver's end-to-end check sees.
  - `Script`: samples delivered once at set times, for event inputs.
  - `HpcStub`: the HPC-to-island rows of design V1.0 section 8.1 that `H7`
    (heartbeat), `H2` (capability), `H5` (trajectory) and `H1` (forwarded
    Autoware topics) produce, at their rates, with faults per topic or for
    the whole link (`inject_link`, an `H1` fault). It models timing and
    end-to-end state; payloads are `()`.
  - `SECTION_3_INPUTS`: every input of the WP2 nodes with its edge and
    rate, so a node test knows what to script.
  - `Runner` and `Rig`: the runner steps a `StepNode` at a fixed period in
    simulated time; the test's `Rig` builds each step's input from its
    publishers and records inputs and outputs in a `Trace`.
  - `Trace` and `assert_golden`: a trace renders as one line per event,
    `<time ms> <source> <event>`; `assert_golden` compares it with a file
    in `tests/golden/`, skipping `#` comments and blank lines, and
    `SI_BLESS_GOLDEN=1` rewrites the file from the run.
- Tests: `tests/hpc_loss.rs` is the example, the D1 HPC-loss scenario
  (stop, freeze, corrupt) against a placeholder heartbeat watch with golden
  traces; `tests/scripted_inputs.rs` scripts every WP2 input;
  `tests/fault_code_table.rs` checks `si_core::FaultCode` against
  `FaultEvent.msg`.

## Writing a node test with the harness

1. Implement `Rig` for the node: in `inputs`, take what each publisher,
   script and the `HpcStub` delivered up to `now` and build the node's
   `Input`; record what the golden trace should show. In `outputs`, record
   the node's decisions.
2. Inject the faults of the scenario, run with `Runner::run_until`, assert
   the timing claims on the trace (`Trace::first`), then `assert_golden`.
3. Create the golden file with `SI_BLESS_GOLDEN=1 cargo test -p <crate>`,
   read it, add a `#` header that says what the scenario shows, commit.

Time is simulated, so a run is deterministic and measures no timing; the
harness drives step functions, not images.

## Design notes

The only shared-code consumer allowed to use `std` besides the X5H A720AE side (ARCHITECTURE.md rule 4).

## Build and test

```bash
cargo test -p si-platform-host
```
