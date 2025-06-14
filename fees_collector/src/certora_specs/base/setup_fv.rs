// Init valid environment and ghost state from storage

#[macro_export]
macro_rules! setup_fv {
    ($e:expr) => {
        cvlr::cvlr_assume!($e.ledger().timestamp() != 0);
        crate::certora_specs::base::ghost_init::initialize_ghost_state_from_storage(&$e);
    };
}