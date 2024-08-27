#[cfg(test)]
mod tests {

    use envman::EnvMan;

    #[derive(EnvMan, Debug, PartialEq)]
    #[allow(unused)]
    struct Foo {
        f0: i32,
        #[envman(rename = "f1")]
        f_n: String,
    }

    #[test]
    fn test() {
        unsafe {
            std::env::set_var("F0", "5");
            std::env::set_var("f1", "saikou");
        }
        assert_eq!(
            Foo::load().unwrap(),
            Foo {
                f0: 5,
                f_n: String::from("saikou")
            }
        );
    }
}
