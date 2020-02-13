#[test]
fn test_filename_extraction() {
    use super::bare_name;
    assert_eq!(Some("my_function".to_owned()), bare_name("Development/releases/my_function.zip"));
    assert_eq!(Some("other Func_1.0.0.23".to_owned()), bare_name("other Func_1.0.0.23.zip"));
    assert_eq!(Some("chrząszcz_game_0.32.2-x86_64-linux".to_owned()), bare_name("/dir/subdir/Sub sub dir/chrząszcz_game_0.32.2-x86_64-linux.zip"));
    assert_eq!(None, bare_name("Docs/not_a_zip.xls"));
    assert_eq!(None, bare_name("also not a zip"));
}

#[test]
fn test_path_extraction() {
    use super::dir_name;
    assert_eq!("", dir_name("top-level file.png"));
    assert_eq!("dir", dir_name("/dir/file.tar.gz"));
    assert_eq!("tsu/util", dir_name("tsu/util/video.webm"));
}