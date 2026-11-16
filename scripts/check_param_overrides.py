"""Validate the deployment parameter files of the bringups against node schemas.

A node package declares its parameters once, in schema/<node>.schema.json,
with defaults in config/<node>.param.yaml (Autoware's convention, checked by
json-schema-check). A bringup overrides them per branch in
config/<branch>/<node>.param.yaml. This script checks each such override
against the schema of the same name, so an override cannot set a parameter
the node does not declare.

Usage: check_param_overrides.py [REPO_ROOT]
"""

import pathlib
import subprocess
import sys


def main(argv):
    root = pathlib.Path(argv[1] if len(argv) > 1 else ".").resolve()
    schemas = {}
    for schema in root.glob("src/**/schema/*.schema.json"):
        schemas.setdefault(schema.name.removesuffix(".schema.json"), []).append(schema)
    failed = False
    overrides = sorted(root.glob("src/*_bringup/config/*/*.param.yaml"))
    for override in overrides:
        node = override.name.removesuffix(".param.yaml")
        found = schemas.get(node, [])
        if len(found) != 1:
            print(f"FAIL {override.relative_to(root)}: {len(found)} schemas named {node}")
            failed = True
            continue
        result = subprocess.run(
            ["check-jsonschema", "--schemafile", str(found[0]), str(override)],
            capture_output=True,
            text=True,
        )
        status = "ok  " if result.returncode == 0 else "FAIL"
        print(f"{status} {override.relative_to(root)} against {found[0].relative_to(root)}")
        if result.returncode != 0:
            print(result.stdout + result.stderr)
            failed = True
    print(f"{len(overrides)} override file(s) checked")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
