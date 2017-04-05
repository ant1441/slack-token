macro_rules! printlist {
    ( $token:ident ) => {
        return format_list(None, &*$token.read().map_err(|_| "unable to lock token (r)")?);
    };
    ( $token:ident, $fmt:expr, $($arg:tt)*) => {
        return format_list(Some(format!($fmt, $($arg)*)), &*$token.read().map_err(|_| "unable to lock token (r)")?);
    };
}
