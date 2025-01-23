use uuid::Uuid;

#[derive(Debug)]
pub struct CleanUp {
    pub path_prefix: String,
}

impl Default for CleanUp {
    fn default() -> Self {
        let uuid = Uuid::new_v4().to_string();
        let path = format!("/tmp/cryptpay/{uuid}");
        let exists = std::path::Path::new(&path).exists();
        if !exists {
            std::fs::create_dir_all(&path).unwrap();
        }
        Self { path_prefix: path }
    }
}

impl CleanUp {
    pub fn new(path_prefix: String) -> Self {
        let exists = std::path::Path::new(&path_prefix).exists();
        if !exists {
            std::fs::create_dir(&path_prefix).unwrap();
        }
        Self { path_prefix }
    }
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        let path_prefix = &self.path_prefix;
        let exists = std::path::Path::new(&path_prefix).exists();
        if exists {
            std::fs::remove_dir_all(path_prefix).unwrap();
        }
        println!("Workspace cleaned.")
    }
}
