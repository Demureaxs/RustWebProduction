pub trait Create {
    fn create(&self, title: &str) {
        println!("Creating a new task: {}", title);
    }
}
