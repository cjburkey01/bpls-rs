#[macro_export]
macro_rules! try_consume_token {
    ($slf:expr, $c1:ident, ( $($ptrn:pat)|* ) => $action:stmt) => {{
        loop {
            if let Some($c1) = $slf.c1 {
                match $c1.1 {
                    ( $($ptrn)|* ) => {
                        $action
                        $slf.next_char();
                    }
                    _ => break,
                }
            } else {
                break;
            }
        };
    }};
    ($slf:expr, $c1:ident, $ptrn:pat => $action:stmt) => {
        $crate::try_consume_token!($slf, $c1, ($ptrn) => $action);
    };
}
