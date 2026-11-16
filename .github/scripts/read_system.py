"""Print the ROS 2 edition and the RMW the bringup's system.toml selects.

`[system] ros_edition` and `[system] rmw` in
src/autoware_safety_island_bringup/system.toml are the one switch for each;
CI reads them here instead of naming an edition or an RMW itself. Only the
two `key = "value"` lines of the `[system]` table are read, so the runner
needs no TOML library.

Usage: read_system.py [SYSTEM_TOML]   (writes key=value lines to stdout)
"""

import re
import sys

EDITIONS = {"humble", "jazzy"}
RMW_NAMES = {"zenoh", "xrce", "cyclonedds"}
DEFAULT = "src/autoware_safety_island_bringup/system.toml"
TABLE = re.compile(r"^\s*\[(?P<name>[^\]]+)\]\s*(?:#.*)?$")
STRING = re.compile(r'^\s*(?P<key>[A-Za-z0-9_]+)\s*=\s*"(?P<value>[^"]*)"\s*(?:#.*)?$')


def system_table(lines):
    """Return the string keys of the [system] table."""
    values = {}
    in_system = False
    for line in lines:
        table = TABLE.match(line)
        if table:
            in_system = table.group("name").strip() == "system"
            continue
        match = STRING.match(line)
        if in_system and match:
            values[match.group("key")] = match.group("value")
    return values


def read(path):
    """Return (ros_edition, rmw) from the [system] table of path."""
    with open(path, encoding="utf-8") as stream:
        system = system_table(stream.read().splitlines())
    edition = system.get("ros_edition", "humble")
    rmw = system.get("rmw", "zenoh")
    if edition not in EDITIONS:
        raise ValueError(f"{path}: ros_edition {edition!r} is not one of {sorted(EDITIONS)}")
    if rmw not in RMW_NAMES:
        raise ValueError(f"{path}: rmw {rmw!r} is not one of {sorted(RMW_NAMES)}")
    return edition, rmw


def main(argv):
    edition, rmw = read(argv[1] if len(argv) > 1 else DEFAULT)
    print(f"ros_edition={edition}")
    print(f"rmw={rmw}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
