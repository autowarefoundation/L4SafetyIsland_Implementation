# Island-internal: N3 si_state_machine's mode, for I2 command_mode_decider and
# I7 mrm_handler (WP0-WP3-SafetyFloor.md section 1.1). The same states as
# IslandState.mode (design V1.0 section 6.1); N3 is their single owner.

uint8 MODE_BOOT = 0
uint8 MODE_SELF_TEST = 1
uint8 MODE_STANDBY = 2
uint8 MODE_ADS_ACTIVE = 3
uint8 MODE_DEGRADED = 4
uint8 MODE_LANE_KEEP_HOLD = 5
uint8 MODE_MRM_PREPARE = 6
uint8 MODE_MRM_EXECUTE = 7
uint8 MODE_MRM_COMPLETE = 8
uint8 MODE_EMERGENCY_STOP = 9
uint8 MODE_SAFE_STOP = 10

builtin_interfaces/Time stamp
uint8 mode
