mod common;

#[test]
fn call_stack_tree_json_representation_preserves_structure(
) -> Result<(), Box<dyn std::error::Error>> {
    let tree = common::sample_call_stack_tree();
    let json = tree.json_representation()?;

    assert!(json.contains("callStacks"));
    assert!(tree.as_value()["callStacks"].is_array());
    Ok(())
}
