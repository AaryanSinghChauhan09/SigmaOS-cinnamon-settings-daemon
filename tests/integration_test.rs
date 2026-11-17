// Integration test for SigmaOS-cinnamon-settings-daemon
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_cinnamon_settings_daemon as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
