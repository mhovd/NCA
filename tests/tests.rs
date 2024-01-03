use nca::data::Data;

#[test]
fn test_auc_calculation() {
    let data = Data::new(
        Some("001".to_string()),
        vec![0.0, 1.0, 2.0, 3.0],
        vec![0.0, 2.0, 4.0, 6.0],
        100.0,
    );
    let result = data.calculate_auc();
    
    // Asserting based on the result's Ok and Err variants
    match result {
        Ok(auc) => {
            let expected_auc = 9.0;
            assert_eq!(
                auc, expected_auc,
                "AUC calculation did not match expected value"
            );
        },
        Err(e) => panic!("Failed to calculate AUC: {}", e),
    }
}
