pub fn search_mode_name_to_op(name: &str) -> &'static str {
    match name {
        "exact" => "=",
        "ancestor" => "@>",
        "descendant" => "<@",
        _ => "<@",
    }
}
