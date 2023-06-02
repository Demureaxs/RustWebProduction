pub trait Delete {
    fn delete(&self, title: &str) {
        println!("Deleting {}", title);
    }
}