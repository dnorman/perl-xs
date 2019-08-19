use perl_xs::{HV, SV};

package!("XSTest::Hash");

#[perlxs]
fn test_fetch(hv: HV, key: SV) -> Option<SV> {
    hv.fetch::<SV>(&key.to_string().unwrap())
}

#[perlxs]
fn test_store(hv: HV, key: SV, val: SV) {
    hv.store(&key.to_string().unwrap(), val);
}

#[perlxs]
fn test_exists(hv: HV, sv: SV) -> bool {
    hv.exists(&sv.to_string().unwrap())
}

#[perlxs]
fn test_clear(hv: HV) {
    hv.clear();
}

#[perlxs]
fn test_delete(hv: HV, sv: SV) -> Option<SV> {
    hv.delete::<SV>(&sv.to_string().unwrap())
}
