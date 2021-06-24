#[cfg(test)]
use crate::bootstrap::PackageVersion;

#[test]
#[allow(non_snake_case)]
fn package_version_zero_produces_zero_major_minor() {
    let v = PackageVersion {
        major: 0,
        minor: 0,
        revision: 0,
        build: 0,
    };
    assert_eq!(v.to_major_minor(), 0);
}

#[test]
fn package_version_max_produces_correct_major_minor() {
    let v = PackageVersion {
        major: 255,
        minor: 255,
        revision: 42,
        build: 42,
    };
    assert_eq!(v.to_major_minor(), 255 << 8 | 255);
}

#[test]
fn package_version_mixed_produces_correct_major_minor() {
    let v = PackageVersion {
        major: 0,
        minor: 8,
        revision: 42,
        build: 192,
    };
    assert_eq!(v.to_major_minor(), 0 << 8 | 8);
}

#[test]
fn package_version_has_expected_c_representation() {
    let v = PackageVersion {
        major: 1,
        minor: 2,
        build: 3,
        revision: 4,
    };
    unsafe {
        let bytes = &v as *const _ as *const u8;
        assert_eq!(std::slice::from_raw_parts(bytes, 4), [4, 3, 2, 1]);
    }
}
