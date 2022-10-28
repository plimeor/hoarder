use crate::utils::env_ext;

pub fn expand(path_like: String) -> String {
    if path_like.starts_with("~/") {
        path_like.replacen("~", &env_ext::home_dir(), 1)
    } else {
        path_like
    }
}
