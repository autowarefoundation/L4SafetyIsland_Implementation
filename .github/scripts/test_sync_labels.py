"""Tests of sync_labels.py: python3 -m unittest discover -s .github/scripts."""

import unittest

import sync_labels


def label(name, replaces=(), color="ededed", description="d"):
    return {"name": name, "color": color, "description": description, "replaces": list(replaces)}


def existing(*names, color="ededed", description="d"):
    return {name: {"color": color, "description": description} for name in names}


class PlanTest(unittest.TestCase):
    def test_creates_a_new_label(self):
        steps = sync_labels.plan([label("run:integration")], {})
        self.assertEqual([("create", label("run:integration"))], steps)

    def test_renames_the_old_label_so_issues_keep_it(self):
        steps = sync_labels.plan([label("wp:0", ["wp0"])], existing("wp0"))
        self.assertEqual([("rename", "wp0", label("wp:0", ["wp0"]))], steps)

    def test_leaves_a_matching_label_alone(self):
        self.assertEqual([], sync_labels.plan([label("type:bug")], existing("type:bug")))

    def test_updates_color_and_description(self):
        steps = sync_labels.plan([label("type:bug", color="d73a4a")], existing("type:bug"))
        self.assertEqual("update", steps[0][0])

    def test_color_compares_without_case(self):
        steps = sync_labels.plan(
            [label("type:bug", color="D73A4A")], existing("type:bug", color="d73a4a")
        )
        self.assertEqual([], steps)

    def test_old_and_new_both_present_is_a_conflict(self):
        steps = sync_labels.plan([label("type:bug", ["bug"])], existing("type:bug", "bug"))
        self.assertEqual(["conflict"], [step[0] for step in steps])

    def test_two_old_labels_present_is_a_conflict(self):
        steps = sync_labels.plan(
            [label("type:task", ["work-item", "task"])], existing("work-item", "task")
        )
        self.assertEqual(["conflict"], [step[0] for step in steps])


class LeftoversTest(unittest.TestCase):
    def test_reports_labels_the_file_does_not_know(self):
        defined = [label("type:bug", ["bug"])]
        self.assertEqual(
            ["duplicate"], sync_labels.leftovers(defined, existing("bug", "duplicate"))
        )


class GhCommandTest(unittest.TestCase):
    def test_rename_uses_label_edit_with_the_new_name(self):
        cmd = sync_labels.gh_command(("rename", "wp0", label("wp:0", ["wp0"])), "o/r")
        self.assertEqual(["gh", "label", "edit", "wp0", "--repo", "o/r", "--name", "wp:0"], cmd[:8])


class LabelsFileTest(unittest.TestCase):
    def test_the_repository_file_loads_and_each_old_name_is_replaced_once(self):
        defined = sync_labels.load_defined(sync_labels.LABELS_FILE)
        olds = [old for item in defined for old in item.get("replaces", [])]
        self.assertEqual(len(olds), len(set(olds)))
        for item in defined:
            self.assertRegex(item["color"], r"^[0-9a-f]{6}$")


if __name__ == "__main__":
    unittest.main()
