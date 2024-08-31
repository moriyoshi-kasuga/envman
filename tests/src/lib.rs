#[cfg(test)]
mod tests {
    use envman::EnvMan;

    #[derive(EnvMan, Debug, PartialEq)]
    struct Normal {
        db_url: String,
        db_max_conn: u8,
    }

    #[test]
    fn normal() {
        unsafe {
            std::env::set_var("DB_URL", "mysql://example");
            std::env::set_var("DB_MAX_CONN", "5");
        }
        assert_eq!(
            Normal::load().unwrap(),
            Normal {
                db_url: String::from("mysql://example"),
                db_max_conn: 5
            }
        );
    }

    #[derive(EnvMan, Debug, PartialEq)]
    struct Rename {
        #[envman(rename = "CORE_DB_URL")]
        db_url_1: String,
        #[envman(rename = "TRANSACTION_DB_URL")]
        db_url_2: String,
    }

    #[test]
    fn rename() {
        unsafe {
            std::env::set_var("CORE_DB_URL", "mysql://example.1");
            std::env::set_var("TRANSACTION_DB_URL", "mysql://example.2");
        }
        assert_eq!(
            Rename::load().unwrap(),
            Rename {
                db_url_1: String::from("mysql://example.1"),
                db_url_2: String::from("mysql://example.2")
            }
        );
    }

    #[derive(EnvMan, Debug, PartialEq)]
    struct TestDefault {
        redis_url: String,
        #[envman(default = "5")]
        redis_max_conn: u8,
    }

    #[test]
    fn default() {
        unsafe {
            std::env::set_var("REDIS_URL", "redis://example");
        }
        assert_eq!(
            TestDefault::load().unwrap(),
            TestDefault {
                redis_url: String::from("redis://example"),
                redis_max_conn: 5
            }
        );
    }

    #[derive(EnvMan, Debug, PartialEq)]
    struct TestOption {
        secret_1: Option<String>,
        #[envman(default = "5")]
        secret_2: Option<String>,
    }

    #[test]
    fn option() {
        assert_eq!(
            TestOption::load().unwrap(),
            TestOption {
                secret_1: None,
                secret_2: Some(String::from("5"))
            }
        );
    }

    #[derive(EnvMan, Debug, PartialEq)]
    struct TestTestValue {
        #[envman(test = "2")]
        secret_1: String,
        #[envman(default = "1", test = "3")]
        secret_2: String,
    }

    #[test]
    fn test_value() {
        assert_eq!(
            TestTestValue::load().unwrap(),
            TestTestValue {
                secret_1: String::from("2"),
                secret_2: String::from("3")
            }
        );
    }
}
