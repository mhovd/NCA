use nca::data::Data;

#[test]
fn test_auc_calculation() {
    let data = Data::new(
        Some("001".to_string()),
        vec![0.0, 1.0, 2.0, 3.0],
        vec![0.0, 2.0, 4.0, 6.0],
        100.0,
    );
    let auc = data.calculate_auc();
    dbg!(&auc);
    let expected_auc = 9.0;
    assert_eq!(
        auc, expected_auc,
        "AUC calculation did not match expected value"
    );
}