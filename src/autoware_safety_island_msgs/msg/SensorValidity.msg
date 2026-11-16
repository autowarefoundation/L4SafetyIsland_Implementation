# Island-internal: N17 si_sensor_health's validity verdict on one sensor, for
# N8 si_fault_manager and N16 si_safety_world_model (WP0-WP3-SafetyFloor.md
# section 1.1). Same sensor ids as SensorHealth.

uint8 SENSOR_CAMERA = 0
uint8 SENSOR_RADAR = 1
uint8 SENSOR_ULTRASONIC = 2
uint8 SENSOR_IMU_GNSS = 3
uint8 SENSOR_LIDAR = 4

builtin_interfaces/Time stamp

uint8 sensor

# False once the sensor is lost (silent past its age limit), blocked or
# reported invalid.
bool valid

# Fault code for N8's table (FaultEvent.CODE_*; 0 = none).
uint32 fault_code
