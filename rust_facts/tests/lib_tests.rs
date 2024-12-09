#[test]
fn add_happy_path_result_is_42() {
    // given
    let a = 40;
    let b = 2;
    let expected_result = 42;

    // when
    let result = rust_facts::bla::add(a,b);
    
    // then
    assert_eq!(result, expected_result);
}
