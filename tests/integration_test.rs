//! Integration tests for SigmaOS-cinnamon-settings-daemon.

use sigmaos_cinnamon_settings_daemon_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
