"""Tests of claim.py: python3 -m unittest discover -s .github/scripts."""

import datetime
import unittest

import claim

NOW = datetime.datetime(2026, 12, 1, tzinfo=datetime.timezone.utc)


def days_ago(days):
    return NOW - datetime.timedelta(days=days)


def assigned(login, actor, days):
    return {"assignee": login, "actor": actor, "created_at": days_ago(days)}


class ParseCommandTest(unittest.TestCase):
    def test_claim_and_unclaim(self):
        self.assertEqual("claim", claim.parse_command("/claim"))
        self.assertEqual("unclaim", claim.parse_command("  /Unclaim \nthanks"))

    def test_first_non_empty_line_decides(self):
        self.assertEqual("claim", claim.parse_command("\n\n/claim\n"))
        self.assertIsNone(claim.parse_command("I will /claim this"))

    def test_other_words_are_not_commands(self):
        self.assertIsNone(claim.parse_command("/claimed"))
        self.assertIsNone(claim.parse_command(""))
        self.assertIsNone(claim.parse_command(None))


class DecideClaimTest(unittest.TestCase):
    def test_ready_and_unassigned_is_assigned(self):
        assign, message = claim.decide_claim("open", ["status:ready"], [], "alice")
        self.assertTrue(assign)
        self.assertIn("@alice", message)

    def test_not_ready_is_refused(self):
        assign, _ = claim.decide_claim("open", ["status:needs-design"], [], "alice")
        self.assertFalse(assign)

    def test_assigned_issue_is_refused(self):
        assign, message = claim.decide_claim("open", ["status:ready"], ["bob"], "alice")
        self.assertFalse(assign)
        self.assertIn("@bob", message)

    def test_closed_issue_is_refused(self):
        assign, _ = claim.decide_claim("closed", ["status:ready"], [], "alice")
        self.assertFalse(assign)


class DecideUnclaimTest(unittest.TestCase):
    def test_assignee_releases(self):
        self.assertTrue(claim.decide_unclaim(["alice"], "alice")[0])

    def test_others_cannot_release(self):
        self.assertFalse(claim.decide_unclaim(["alice"], "bob")[0])


class ExpiredClaimsTest(unittest.TestCase):
    def test_bot_claim_expires_after_the_limit(self):
        events = [assigned("alice", "github-actions", 14)]
        self.assertEqual(["alice"], claim.expired_claims(["alice"], events, False, NOW))

    def test_rest_bot_login_counts_as_the_bot(self):
        events = [assigned("alice", "github-actions[bot]", 20)]
        self.assertEqual(["alice"], claim.expired_claims(["alice"], events, False, NOW))

    def test_young_claim_stays(self):
        events = [assigned("alice", "github-actions", 13)]
        self.assertEqual([], claim.expired_claims(["alice"], events, False, NOW))

    def test_open_linked_pull_request_keeps_the_claim(self):
        events = [assigned("alice", "github-actions", 30)]
        self.assertEqual([], claim.expired_claims(["alice"], events, True, NOW))

    def test_assignment_by_a_person_never_expires(self):
        events = [assigned("alice", "maintainer", 30)]
        self.assertEqual([], claim.expired_claims(["alice"], events, False, NOW))

    def test_latest_assignment_counts(self):
        events = [assigned("alice", "github-actions", 30), assigned("alice", "maintainer", 2)]
        self.assertEqual([], claim.expired_claims(["alice"], events, False, NOW))

    def test_reclaim_restarts_the_clock(self):
        events = [assigned("alice", "github-actions", 30), assigned("alice", "github-actions", 1)]
        self.assertEqual([], claim.expired_claims(["alice"], events, False, NOW))

    def test_only_current_assignees_expire(self):
        events = [assigned("alice", "github-actions", 30)]
        self.assertEqual([], claim.expired_claims(["bob"], events, False, NOW))


if __name__ == "__main__":
    unittest.main()
