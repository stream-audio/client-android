macro_rules! log_and_ignore_err {
    ($e:expr) => {{
        let res = $e;
        match res {
            Ok(_) => {}
            Err(e) => {
                log::error!("An error occurred: {}, during: {}", e, stringify!($e));
            }
        }
    }};
    ($e:expr, $ctx:expr) => {{
        let res = $e;
        match res {
            Ok(_) => {}
            Err(e) => {
                log::error!("An error occurred: {}, during: {}", e, $ctx);
            }
        }
    }};
}

macro_rules! log_err {
    ($e:expr) => {{
        let res = $e;
        match res {
            Ok(d) => d,
            Err(e) => {
                log::error!("An error occurred: {}, during: {}", e, stringify!($e));
                return;
            }
        }
    }};
    ($e:expr; $ctx:expr) => {{
        let res = $e;
        match res {
            Ok(d) => d,
            Err(e) => {
                log::error!("An error occurred: {}, during: {}", e, $ctx);
                return;
            }
        }
    }};
}
