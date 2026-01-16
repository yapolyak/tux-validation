use std::io::Cursor;
use tux_validation::os_release;

#[test]
fn read_os_id_and_codename() {
    let mock_data = r#"
ID=debian
VERSION_CODENAME="forky"
# This is a comment
        EXTRA_VAR=value
    "#;

    let reader = Cursor::new(mock_data);
    let result = os_release::parse_os_release_from_reader(reader).unwrap();

    assert_eq!(result.get("ID").unwrap(), "debian");
    assert_eq!(result.get("VERSION_CODENAME").unwrap(), "forky");
    assert_eq!(result.get("EXTRA_VAR").unwrap(), "value");
}
