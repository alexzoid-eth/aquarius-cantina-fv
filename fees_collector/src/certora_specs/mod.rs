pub(crate) mod base;
pub(crate) mod harnesses;
pub(crate) mod sanity;
pub(crate) mod state_transition;

use crate::certora_specs::base::ParametricParams;
use crate::certora_specs::base::ghost_log::ghost_log_all;

use crate::contract::FeesCollector;
use crate::interface::AdminInterface;
use access_control::interface::TransferableContract;
use upgrade::interface::UpgradeableContract;

use soroban_sdk::{Address, BytesN, Env, Symbol, Vec};

use cvlr_soroban_derive::rule;

use crate::{setup_fv, parametric_rule};

use crate::certora_specs::sanity::sanity;
use crate::certora_specs::state_transition::{
    state_transition_admin_deadline_lifecycle,
    state_transition_em_admin_deadline_lifecycle,
    state_transition_upgrade_deadline_lifecycle,
    state_transition_admin_role,
    state_transition_emergency_admin_role,
    state_transition_future_admin_consistency,
    state_transition_future_em_admin_consistency,
    state_transition_future_wasm_consistency,
    state_transition_emergency_mode_behavior,
    state_transition_admin_init_once,
};

// Generate all sanity rules
parametric_rule!(sanity);

// Generate rules in parametric style for each variables transition property
parametric_rule!(state_transition_admin_deadline_lifecycle);
parametric_rule!(state_transition_em_admin_deadline_lifecycle);
parametric_rule!(state_transition_upgrade_deadline_lifecycle);
parametric_rule!(state_transition_admin_role);
parametric_rule!(state_transition_emergency_admin_role);
parametric_rule!(state_transition_future_admin_consistency);
parametric_rule!(state_transition_future_em_admin_consistency);
parametric_rule!(state_transition_future_wasm_consistency);
parametric_rule!(state_transition_emergency_mode_behavior);
parametric_rule!(state_transition_admin_init_once);
