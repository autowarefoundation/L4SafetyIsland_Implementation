# Island-internal: N2 si_hpc_supervisor's recovery verdict for N3
# si_state_machine (design V1.0 section 6.1, N2). During a B2 hold, N2
# re-qualifies a returning HPC over the declared confirmation window:
# heartbeat restored and trajectories valid, fresh and in-envelope. N3 may
# take the hand-back edge LANE_KEEP_HOLD -> ADS_ACTIVE only on `requalified`.

builtin_interfaces/Time stamp

# True once the HPC has met every condition for the whole window.
bool requalified

# How long the conditions have held so far [s]; 0 while any is unmet.
float32 confirmation_elapsed
