use crate::some_trait::SomeTrait;

/// # That is *my* Bar
pub struct Bar {

}

impl Bar {
    /// # This is test
    pub fn test(&self) {
        println!("Bar baby!");
    }
}

impl SomeTrait for Bar {
}