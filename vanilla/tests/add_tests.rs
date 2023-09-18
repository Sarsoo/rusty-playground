mod scaffold;

#[test]
fn add_test_1() {
    scaffold::setup();
    assert_eq!(4, 2 + 2);
}