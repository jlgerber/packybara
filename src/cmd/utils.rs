pub fn extract_coords<'a>(
    level: &'a Option<String>,
    role: &'a Option<String>,
    platform: &'a Option<String>,
    site: &'a Option<String>,
    mode: &'a Option<String>,
) -> (String, String, String, String, String) {
    let r = role.clone().unwrap_or("any".to_string());
    let l = level.clone().unwrap_or("facility".to_string());
    let p = platform.clone().unwrap_or("any".to_string());
    let s = site.clone().unwrap_or("any".to_string());
    let m = mode.clone().unwrap_or("ancestor".to_string());

    (l, r, p, s, m)
}
