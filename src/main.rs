use crate::some_trait::SomeTrait;

mod bar;
mod some_trait;

fn main() {
    let x = bar::Bar{};
    x.test();
    x.boom();
}