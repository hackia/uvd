#[must_use]
pub fn get_licenses() -> Vec<String> {
    let mut licenses = vec![
        String::from("MIT"),
        String::from("GPL-3.0"),
        String::from("AGPL-3.0"),
        String::from("Apache-2.0"),
        String::from("GPL-2.0"),
        String::from("BSD-3-Clause"),
        String::from("BSD-2-Clause"),
        String::from("MPL-2.0"),
        String::from("EPL-2.0"),
        String::from("Unlicense"),
        String::from("CC0-1.0"),
        String::from("Zlib"),
        String::from("Apache-2.0"),
        String::from("CC-BY-4.0"),
    ];
    licenses.sort();
    licenses
}
