use super::*;

#[test]
fn dates() {
	assert_eq!(date(0), "1970-01-01");
	assert_eq!(date(951_782_400_000), "2000-02-29");
	assert_eq!(date(1_793_145_600_000), "2026-10-28");
}

#[test]
fn decodes_paths() {
	assert_eq!(percent_decode("%C3%BCber-uns/team"), "über-uns/team");
}
