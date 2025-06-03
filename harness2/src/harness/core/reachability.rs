use std::collections::{HashMap, HashSet};

use super::{process::ProcessID, state_machine::StateMachineNodeID};

pub struct ProcessStateReachability {
    active_states: HashMap<ProcessID, HashSet<StateMachineNodeID>>,
    cooccuring_state_pairs: HashMap<(ProcessID, ProcessID), HashSet<(StateMachineNodeID, StateMachineNodeID)>>
}

impl ProcessStateReachability {
    pub fn new() -> ProcessStateReachability {
        ProcessStateReachability {
            active_states: HashMap::new(),
            cooccuring_state_pairs: HashMap::new()
        }
    }

    pub fn get_active_states(&self, process: ProcessID) -> Option<&HashSet<StateMachineNodeID>> {
        self.active_states.get(&process)
    }

    pub fn get_cooccuring_states(&self, process: ProcessID, other_process: ProcessID) -> Option<&HashSet<(StateMachineNodeID, StateMachineNodeID)>> {
        self.cooccuring_state_pairs.get(&(process, other_process))
    }

    pub fn mark_active(&mut self, process: ProcessID, state: StateMachineNodeID) {
        self.active_states.entry(process)
            .or_default()
            .insert(state);
    }

    pub fn mark_cooccuring(&mut self, process: ProcessID, state: StateMachineNodeID, other_process: ProcessID, other_state: StateMachineNodeID) {
        self.mark_active(process, state);
        self.mark_active(other_process, other_state);

        self.cooccuring_state_pairs.entry((process, other_process))
            .or_default()
            .insert((state, other_state));
        self.cooccuring_state_pairs.entry((other_process, process))
            .or_default()
            .insert((other_state, state));
    }
}
