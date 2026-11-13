//! One directory and one table of send-as cases, shared by the directory,
//! SMTP submission, REST API and JMAP tests, so the four surfaces are held
//! to the same answers.
//!
//! The rule being pinned (`Directory::owns_address`): an account may send as
//! an address only when it is stored for that account, or when it is an
//! enabled multi-target alias the account may send as (a member, or a listed
//! sender when the alias names senders), or an enabled mask it owns.
//! When `catch_all_send_as` is enabled, the account may send as any address
//! in its catch-all domains that is not owned by another account or alias.
//! Case never matters. Everything that merely *delivers* to the account is not
//! enough: a `+tag` sub-address and a domain alias resolve for delivery but
//! are refused for sending.

use crate::smtp::directory::{AliasSpec, Directory};

/// A send-as case: the account, the address it tries to send as, and
/// whether that is allowed.
pub(crate) type Case = (&'static str, &'static str, bool);

/// Two hosted domains, `alias.example.org` as a domain alias of
/// `example.org`, `carol` as the catch-all of `example.org` with catch_all_send_as.
pub(crate) fn directory() -> Directory {
	Directory::new(
		["example.org".to_string(), "other.example".to_string()],
		[
			("name.surname@example.org", "alice"),
			("alice@example.org", "alice"),
			("alice@other.example", "alice"),
			("bob@example.org", "bob"),
			("bob.jones@example.org", "bob"),
			("carol@example.org", "carol"),
		]
		.map(|(address, account)| (address.to_string(), account.to_string())),
	)
	.with_aliases([
		(
			"team@example.org".to_string(),
			list(&["alice@example.org", "bob@example.org"], &[]),
		),
		(
			"board@example.org".to_string(),
			list(
				&["alice@example.org", "bob@example.org"],
				&["bob@example.org"],
			),
		),
		(
			"ops.list@other.example".to_string(),
			list(&["alice@example.org"], &[]),
		),
	])
	.with_catch_all([("example.org".to_string(), "carol".to_string())])
	.with_catch_all_send_as(["carol"])
	.with_domain_aliases([("alias.example.org".to_string(), "example.org".to_string())])
}

fn list(members: &[&str], senders: &[&str]) -> AliasSpec {
	AliasSpec {
		members: members.iter().map(|m| m.to_string()).collect(),
		senders: senders.iter().map(|s| s.to_string()).collect(),
		hidden: true,
		list_id: None,
	}
}

/// Every case, for every account.
pub(crate) const CASES: &[Case] = &[
	// An address of the account with a dot in the local part, in any case.
	("alice", "name.surname@example.org", true),
	("alice", "Name.Surname@EXAMPLE.org", true),
	("alice", "NAME.SURNAME@example.org", true),
	// The same account in a second hosted domain.
	("alice", "alice@other.example", true),
	// Another account's address, dotted or not, in any case.
	("alice", "bob@example.org", false),
	("alice", "BOB.JONES@example.org", false),
	("bob", "name.surname@example.org", false),
	("carol", "name.surname@example.org", false),
	// The dotted address in a domain where it is not stored.
	("alice", "name.surname@other.example", false),
	// The catch-all account keeps its own address.
	("carol", "carol@example.org", true),
	// An address nobody here owns.
	("alice", "stranger@elsewhere.example", false),
	// A list the account is a member of: any member may send when the list
	// names no senders; in any case.
	("alice", "team@example.org", true),
	("bob", "TEAM@Example.Org", true),
	("carol", "team@example.org", false),
	// A list that names senders: only those, not every member.
	("bob", "board@example.org", true),
	("alice", "board@example.org", false),
	// A dotted list in the other hosted domain.
	("alice", "ops.list@other.example", true),
	("bob", "ops.list@other.example", false),
	// A sub-address delivers to the base account but is not an address the
	// account may send as.
	("alice", "name.surname+news@example.org", false),
	("alice", "alice+news@example.org", false),
	("bob", "alice+news@example.org", false),
	// The catch-all account with catch_all_send_as enabled may send as
	// unknown addresses in its catch-all domain.
	("carol", "unknown@example.org", true),
	("carol", "ANYTHING@example.org", true),
	("carol", "test.user@example.org", true),
	// Other accounts cannot send as catch-all addresses.
	("alice", "unknown@example.org", false),
	("bob", "unknown@example.org", false),
	// A domain alias delivers like its target but is not a sending domain,
	// even for catch_all_send_as.
	("alice", "alice@alias.example.org", false),
	("alice", "name.surname@alias.example.org", false),
	("carol", "unknown@alias.example.org", false),
];
