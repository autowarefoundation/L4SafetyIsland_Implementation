# cspell:ignore cyclonedds
# Environment for every integration step (source it; bash). Installed as /etc/l4si/env.sh
# in the CI images. Outside the images set L4SI_ROS_SETUP, L4SI_AUTOWARE_SETUP and
# L4SI_SWB_SETUP to your own installs first.
# shellcheck shell=bash disable=SC1090

# ROS setup scripts read unset variables; never source them under `set -u`.
_l4si_nounset=0
case $- in *u*)
    _l4si_nounset=1
    set +u
    ;;
esac

source "${L4SI_ROS_SETUP:-/opt/ros/${ROS_DISTRO:-humble}/setup.bash}"
source "${L4SI_AUTOWARE_SETUP:-/opt/autoware/setup.bash}"
source "${L4SI_SWB_SETUP:-/opt/swb-ws/install/setup.bash}"

# swb's stack sets its own DDS profile (loopback unicast) for every process it starts.
# Autoware's image exports a CYCLONEDDS_URI of its own; drop it so nothing else
# picks it up by accident.
unset CYCLONEDDS_URI
export RMW_IMPLEMENTATION=rmw_cyclonedds_cpp
# Domain 0 stays empty (swb's rule): a stray ros2 command cannot join a run.
export ROS_DOMAIN_ID=0

[ "$_l4si_nounset" = 1 ] && set -u
unset _l4si_nounset
