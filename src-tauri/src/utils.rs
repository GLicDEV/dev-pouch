// use walkdir::WalkDir;

// pub fn list_dir(path: &str) {
//     for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
//         println!("{}", entry.path().display());
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_list_dir() {
//         list_dir("/home/devel/.config/dfx/identity/");
//     }
// }
