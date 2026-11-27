use bottles::{BottleConfig, BottleEnvironment, BottleManager};

#[test]
fn crate_links_and_manages_bottles() {
    let mut mgr = BottleManager::new();
    let bottle = BottleConfig::new("App", BottleEnvironment::Application, "/tmp/app").unwrap();
    mgr.create_bottle(bottle).unwrap();
    assert_eq!(mgr.list_names(), vec!["App"]);
}
