// A non-void JNI call. May return anything — primitives, references, error codes.
// Returns Err if there is a pending exception after the call.
macro_rules! jni_non_void_call {
    ( $jnienv:expr, $name:tt $(, $args:expr )* ) => ({
        trace!("calling checked jni method: {}", stringify!($name));
        let env = $jnienv;

        let res = unsafe {
            jni_method!(env, $name)(env, $($args),*)
        };

        check_exception!(env);
        res
    })
}

// A JNI call that does not check for exceptions or verify
// error codes (if any).
macro_rules! jni_unchecked {
    ( $jnienv:expr, $name:tt $(, $args:expr )* ) => ({
        trace!("calling unchecked jni method: {}", stringify!($name));

        unsafe {
            jni_method!($jnienv, $name)($jnienv, $($args),*)
        }
    })
}

macro_rules! jni_method {
    ( $jnienv:expr, $name:tt ) => {{
        trace!("looking up jni method {}", stringify!($name));
        let env = $jnienv;
        match deref!(deref!(env, "JNIEnv"), "*JNIEnv").$name {
            Some(method) => {
                trace!("found jni method");
                method
            }
            None => {
                trace!("jnienv method not defined, returning error");
                return Err(jni::errors::Error::from(
                    jni::errors::ErrorKind::JNIEnvMethodNotFound(stringify!($name)),
                )
                .into());
            }
        }
    }};
}

macro_rules! check_exception {
    ( $jnienv:expr ) => {
        trace!("checking for exception");
        let check = { jni_unchecked!($jnienv, ExceptionCheck) } == jni::sys::JNI_TRUE;
        if check {
            trace!("exception found, returning error");
            return Err(jni::errors::Error::from(jni::errors::ErrorKind::JavaException).into());
        }
        trace!("no exception found");
    };
}

macro_rules! deref {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            return Err(jni::errors::ErrorKind::NullDeref($ctx).into());
        } else {
            #[allow(unused_unsafe)]
            unsafe {
                *$obj
            }
        }
    };
}
