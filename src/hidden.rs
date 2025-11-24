pub fn is_hidden(path: &std::path::Path) -> bool {
    path.components().any(|comp| {
        if let std::path::Component::Normal(os_str) = comp {
            if let Some(name) = os_str.to_str() {
                return name.starts_with('.');
            }
        }
        false
    })
}

