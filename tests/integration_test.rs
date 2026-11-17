//! Integration tests for SigmaOS-arch-parallax-mapping-kernel.

use sigmaos_arch_parallax_mapping_kernel_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
