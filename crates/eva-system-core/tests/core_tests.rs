//! EVA 系統核心功能自動化測試

use eva_system_core::*;

#[test]
fn test_init_self_inspect_and_summary() {
    // 測試初始化與專案摘要
    let tmp_dir = tempfile::tempdir().expect("建立暫存目錄失敗");
    let db_path = tmp_dir.path().join("eva_test.db");
    let db_path_str = db_path.to_string_lossy();
    let project_root = tmp_dir.path().to_string_lossy();

    // 初始化自省系統
    let db = init_self_inspect(&project_root, &db_path_str)
        .expect("初始化自省系統失敗");

    // 測試 get_project_summary
    let summary = get_project_summary(&db).expect("取得專案摘要失敗");
    assert!(summary.contains("EVA 專案結構"));
}

#[test]
fn test_check_for_changes() {
    // 測試變更檢查
    let tmp_dir = tempfile::tempdir().expect("建立暫存目錄失敗");
    let db_path = tmp_dir.path().join("eva_test2.db");
    let db_path_str = db_path.to_string_lossy();
    let project_root = tmp_dir.path().to_string_lossy();

    let db = init_self_inspect(&project_root, &db_path_str)
        .expect("初始化自省系統失敗");

    // 初始狀態應無變更
    let changed = check_for_changes(&db, &project_root)
        .expect("檢查變更失敗");
    assert!(!changed, "初始專案不應有變更");

    // 模擬新增檔案
    let new_file = tmp_dir.path().join("src").join("foo.rs");
    std::fs::create_dir_all(new_file.parent().unwrap()).unwrap();
    std::fs::write(&new_file, b"// test file").unwrap();

    let changed = check_for_changes(&db, &project_root)
        .expect("檢查變更失敗");
    // 新增檔案後應有變更
    assert!(changed, "新增檔案後應偵測到變更");
}
