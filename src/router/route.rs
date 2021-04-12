struct Route {
    path: String,
    file: String,
    method: String,
}

impl Route {
    pub fn get_path(&self) {
        self.path
    }

    pub fn get_file(&self) {
        self.file
    }

    pub fn get_method(&self) {
        self.method
    }
}