// Init valid environment for properties

#[macro_export]
macro_rules! init_verification {
    ($e:expr) => {
        cvlr::cvlr_assume!($e.ledger().timestamp() != 0);
    };
}