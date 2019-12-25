pub fn search_mode_name_to_op(name: &str) -> &'static str {
    match name {
        "exact" => "=",
        "ancestor" => "@>",
        "descendant" => "<@",
        _ => "<@",
    }
}

pub fn prep_query_str(default: &'static str, value: &str, substitute: bool) -> String {
    let mut result = match value {
        _ if value == default => default.to_string(),
        _ if value.contains("%") => value.to_string(),
        _ => format!("{}.{}", default, value),
    };
    if substitute {
        result = result.replace("_", ".");
    }
    result
}
