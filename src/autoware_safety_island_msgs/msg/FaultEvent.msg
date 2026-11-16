# Island-internal: one fault report, the one type every N8 si_fault_manager
# source publishes (WP0-WP3-SafetyFloor.md sections 1.1 and 1.2).

# Severity, the same scale as FaultVerdict.
uint8 SEVERITY_NONE = 0
uint8 SEVERITY_DEGRADE = 1
uint8 SEVERITY_HPC_LOST = 2
uint8 SEVERITY_BLACKOUT = 3

# The WP3 fault-code table (WP0-WP3-SafetyFloor.md section 1.2). 0 = none.
uint32 CODE_NONE = 0
uint32 CODE_HPC_LIVENESS_LOST = 1
uint32 CODE_HPC_COMMAND_INVALID = 2
uint32 CODE_E2E_VERIFY_FAIL = 3
uint32 CODE_TIME_BASE_INVALID = 4
uint32 CODE_ACTUATOR_NOT_TRACKING = 5
uint32 CODE_SENSING_BLACKOUT = 6
uint32 CODE_FSI_SOC_ERROR = 7

builtin_interfaces/Time stamp

# The reporting node, by its design id number (2 for N2, 17 for N17, ...;
# 100 + n for In).
uint16 source

uint32 code
uint8 severity
