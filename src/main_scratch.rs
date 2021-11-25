use crate::some_trait::SomeTrait;

mod bar;
mod some_trait;
mod sorter;

fn main() {
    let x = bar::Bar{};
    x.test();
    x.boom();

    sorter::run();

    let rslt = add(&2, &3);

    println!("{}", rslt);

}

fn add(a: &i32, b: &i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn add_test()-> Result<(), String> {
        assert_eq!(add(&2, &3), 5);
        Ok(())
    }
}