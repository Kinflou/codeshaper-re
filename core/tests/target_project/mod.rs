// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use codeshaper_core::target_project;
use codeshaper_core::target_project::kinds::text::group::TextGroup;
use codeshaper_core::target_project::kinds::text::TextSolution;
use codeshaper_core::target_project::TargetKind;


#[allow(unused)]
#[test]
fn load_plain_text_target() {
    let Ok(TargetKind::Text(mut target)) = target_project::from_kind_path(
        "plain_text",
        Path::new("/tests/data/test_workspace/target/origin/").to_path_buf(),
    ) else { panic!("Something something wrong type, expected plain text") };

    let expected_files = vec!["hello_world.scl", "hello_new_world.scl"];

    /*
    for (idx, file) in target.file_iter.as_mut().unwrap().enumerate() {
        assert_eq!(file.name(), expected_files[idx]);
    }
    */

    let expected = Box::new(TextSolution::new(
        "Text Solution".to_owned(),
        TextGroup::new(
            &mut target, None, "".to_string(), vec![], vec![]
        ),
    ));

    pretty_assertions::assert_eq!(expected, target)
}
