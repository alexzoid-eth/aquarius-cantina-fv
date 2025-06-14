#!/usr/bin/env python3

import re

# Read the file
with open('/home/zoid/web3/aquarius-cantina-fv/fees_collector/src/certora_specs/base/parametric.rs', 'r') as f:
    content = f.read()

# Define patterns for ParametricParams variables
parametric_params_vars = [
    'caller_admin: Address',
    'caller_emergency_admin: Address', 
    'caller_rewards_admin: Address',
    'caller_operations_admin: Address',
    'caller_pause_admin: Address',
    'caller_emergency_pause_admins: Vec<Address>',
    'role_name: Symbol',
    'new_address: Address',
    'value: bool',
    'address: Address',
    'addresses: Vec<Address>',
    'new_wasm_hash: BytesN<32>'
]

# Ghost state variables (always in this order)
ghost_state_vars = [
    'admin: Option<Address>',
    'emergency_admin: Option<Address>',
    'rewards_admin: Option<Address>',
    'operations_admin: Option<Address>',
    'pause_admin: Option<Address>',
    'emergency_pause_admins: Vec<Address>',
    'admin_transfer_deadline: u64',
    'em_admin_transfer_deadline: u64',
    'future_admin: Option<Address>',
    'future_em_admin: Option<Address>',
    'emergency_mode: bool',
    'upgrade_deadline: u64',
    'future_wasm: Option<BytesN<32>>'
]

# Function to reorder parameters in a function definition
def reorder_function_params(match):
    func_name = match.group(1)
    params_block = match.group(2)
    
    # Extract individual parameters
    param_lines = []
    current_param = ""
    
    for line in params_block.split('\n'):
        line = line.strip()
        if line.endswith(',') or line.endswith(')'):
            current_param += ' ' + line.rstrip(',').rstrip(')')
            if current_param.strip():
                param_lines.append(current_param.strip())
            current_param = ""
        else:
            current_param += ' ' + line
    
    # Separate ParametricParams variables from ghost state variables
    parametric_params = []
    ghost_params = []
    other_params = []
    
    for param in param_lines:
        param = param.strip()
        if param == 'e: Env':
            continue
        
        found_parametric = False
        for pp_var in parametric_params_vars:
            if pp_var in param:
                parametric_params.append(param)
                found_parametric = True
                break
        
        if not found_parametric:
            found_ghost = False
            for gs_var in ghost_state_vars:
                if gs_var in param:
                    ghost_params.append(param)
                    found_ghost = True
                    break
            
            if not found_ghost and param:
                other_params.append(param)
    
    # Build the new parameter list
    new_params = ['e: Env']
    
    if parametric_params:
        new_params.append('// ParametricParams variables')
        new_params.extend(parametric_params)
    
    if ghost_params or other_params:
        new_params.append('// Initialize ghost storage state from parameters')
        # Add ghost state vars in the correct order
        for gs_var in ghost_state_vars:
            for param in ghost_params + other_params:
                if gs_var in param:
                    new_params.append(param)
                    break
    
    # Format the new function definition
    result = f"#[rule]\n            pub fn [< $f _{func_name} >](\n"
    for i, param in enumerate(new_params):
        if param.startswith('//'):
            result += f"                {param}\n"
        else:
            comma = ',' if i < len(new_params) - 1 else ''
            result += f"                {param}{comma}\n"
    result += "            ) {"
    
    return result

# Apply the transformation
pattern = r'#\[rule\]\s+pub fn \[< \$f _(\w+) >\]\(\s*(.*?)\s*\) \{'
content = re.sub(pattern, reorder_function_params, content, flags=re.DOTALL)

# Write the updated content
with open('/home/zoid/web3/aquarius-cantina-fv/fees_collector/src/certora_specs/base/parametric.rs', 'w') as f:
    f.write(content)

print("Parameter reordering completed!")