use error_occur::*;

fn get_option(msg: &str) -> Option<&str> {
    if msg == "" {
        None
    } else {
        Some(msg)
    }
}

fn get_result(msg: &str) -> Result<&str, ErrorRemind> {
    if msg == "" {
        Err(ErrorRemind::new("msg is empty"))
    } else {
        Ok(msg)
    }
}

fn sugar<T>(method: T)
where
    T: Fn() -> std::result::Result<(), ErrorRemind>,
{
    method().unwrap();
}

#[test]
fn use_error_on_option_some() {
    sugar(|| {
        get_option("hello world").error("option error")?;
        Ok(())
    });
}

#[test]
#[should_panic(expected = "error")]
fn use_error_on_option_none() {
    sugar(|| {
        get_option("").error("option error")?;
        Ok(())
    });
}

#[test]
fn use_error_on_result_ok() {
    sugar(|| {
        get_result("hello world").error("result error")?;
        Ok(())
    });
}

#[test]
#[should_panic(expected = "error")]
fn use_error_on_result_err() {
    sugar(|| {
        get_option("").error("result error")?;
        Ok(())
    });
}

#[test]
fn use_error_on_bool_true() {
    sugar(|| {
        true.error("bool error")?;
        Ok(())
    });
}

#[test]
#[should_panic(expected = "error")]
fn use_error_on_bool_false() {
    sugar(|| {
        false.error("bool error")?;
        Ok(())
    });
}

#[test]
#[should_panic(expected = "error")]
fn use_error_on_str() {
    sugar(|| {
        "args error".error("value")?;
        println!("d");
        Ok(())
    });
}

#[test]
#[should_panic(expected = "error")]
fn use_error_on_string() {
    sugar(|| "args error".to_string().error("value"));
}
