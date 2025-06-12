pub(crate) mod base;
pub(crate) mod harnesses;
pub(crate) mod sanity;
pub(crate) mod variables_transition;

use crate::certora_specs::base::ParametricParams;
use crate::certora_specs::base::ghost_log::ghost_log_details;

use crate::contract::FeesCollector;
use crate::interface::AdminInterface;
use access_control::interface::TransferableContract;
use upgrade::interface::UpgradeableContract;

use soroban_sdk::{Address, BytesN, Env, Symbol, Vec};

use cvlr_soroban_derive::rule;

use crate::{setup_fv, parametric_rule};

use crate::certora_specs::sanity::sanity;
use crate::certora_specs::variables_transition::{
    variables_transition_admin_deadline_lifecycle,
    variables_transition_em_admin_deadline_lifecycle,
    variables_transition_upgrade_deadline_lifecycle,
    variables_transition_mutual_exclusion,
    variables_transition_admin_role,
    variables_transition_emergency_admin_role,
    variables_transition_future_admin_consistency,
    variables_transition_future_em_admin_consistency,
    variables_transition_future_wasm_consistency,
    variables_transition_emergency_mode_behavior,
    variables_transition_admin_init_once,
};

// Generate all sanity rules
parametric_rule!(sanity);

// Generate rules in parametric style for each variables transition property
parametric_rule!(variables_transition_admin_deadline_lifecycle);
parametric_rule!(variables_transition_em_admin_deadline_lifecycle);
parametric_rule!(variables_transition_upgrade_deadline_lifecycle);
parametric_rule!(variables_transition_mutual_exclusion);
parametric_rule!(variables_transition_admin_role);
parametric_rule!(variables_transition_emergency_admin_role);
parametric_rule!(variables_transition_future_admin_consistency);
parametric_rule!(variables_transition_future_em_admin_consistency);
parametric_rule!(variables_transition_future_wasm_consistency);
parametric_rule!(variables_transition_emergency_mode_behavior);
parametric_rule!(variables_transition_admin_init_once);
