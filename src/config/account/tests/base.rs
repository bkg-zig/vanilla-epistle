use super::*;

#[test]
fn parses_account() {
	let account: Account = toml::from_str(
		r#"
name = "alice"
addresses = ["alice@example.org", "postmaster@example.org"]
"#,
	)
	.expect("parse account");
	assert_eq!(account.name, "alice");
	assert_eq!(account.addresses.len(), 2);
}

#[test]
fn catch_all_send_as_defaults_to_disabled() {
	let account: Account = toml::from_str(
		r#"
name = "carol"
addresses = ["carol@example.org"]
catch_all = ["example.org"]
"#,
	)
	.expect("parse account");
	assert_eq!(account.catch_all_send_as, None);
}

#[test]
fn parses_catch_all_send_as_switch() {
	let account: Account = toml::from_str(
		r#"
name = "carol"
addresses = ["carol@example.org"]
catch_all = ["example.org"]
catch_all_send_as = true
"#,
	)
	.expect("parse account");
	assert_eq!(account.catch_all_send_as, Some(true));
}

#[test]
fn rejects_unknown_keys() {
	let result: Result<Account, _> = toml::from_str(
		r#"
name = "alice"
addresses = []
quota = "1G"
"#,
	);
	assert!(result.is_err());
}
