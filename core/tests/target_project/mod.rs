// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use codeshaper_core::target_project;
use codeshaper_core::target_project::kinds::text;


#[test]
fn find_kind_initializer() {
    let initializer = target_project::find_initializer("plain_text")
        .unwrap();

    assert_eq!(*initializer as usize, text::TextSolution::from_path_shared as usize);
}

#[allow(unused)]
#[test]
fn load_plain_text_target() {
    let target = target_project::from_kind_path(
        "plain_text", Path::new("/tests/data/test_workspace/target/origin/")
    ).unwrap();
    let mut target_mut = target.borrow_mut();
    let mut file_map = target_mut.file_map().as_mut().unwrap();

    let expected_files = vec!["hello_world.scl", "hello_new_world.scl"];

    let mut idx = 0;
    while let Some(file) = file_map.next(){
        assert_eq!(file.upgrade().unwrap().borrow().name(), expected_files[idx]);
        idx += 1;
    };

    /*
    let expected = TextSolution {
        name: "".to_string(),
        groups: vec![],
        file_map: None,
    };

    pretty_assertions::assert_eq!(target, &TextSolution {
        name: "".to_string(),
        groups: vec![],
        file_map: None,
    })

    assert_eq!(target.name, expected.name);
    assert_eq!(target.groups, expected.groups);
    */
}