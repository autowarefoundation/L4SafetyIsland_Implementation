# HPC -> island heartbeat (design V1.0 section 8.1).
# Producer H7 hpc_heartbeat at 100 Hz; consumer N2 si_hpc_supervisor through
# N1; age limit 20 ms; SI-E2E/B, applied by the transport, not carried here.

uint8 SELF_TEST_UNKNOWN = 0
uint8 SELF_TEST_PASS = 1
uint8 SELF_TEST_FAIL = 2

builtin_interfaces/Time stamp

# Increments by one per publication, so N2 detects a stuck or repeated sender.
uint32 sequence

# Rolling self-test result and its fault code (0 = none).
uint8 self_test
uint32 self_test_fault_code
