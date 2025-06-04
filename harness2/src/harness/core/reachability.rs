use std::collections::{HashMap, HashSet};

use super::{process::ProcessID, state_machine::StateMachineNodeID};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct ProcessReachabilityPair(ProcessID, StateMachineNodeID, ProcessID, StateMachineNodeID);

pub struct ProcessStateReachability {
    active_states: HashMap<ProcessID, HashSet<StateMachineNodeID>>,
    cooccuring_state_pairs: HashMap<(ProcessID, ProcessID), HashSet<(StateMachineNodeID, StateMachineNodeID)>>
}

fn rev2<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

impl ProcessReachabilityPair {
    pub fn new(process: ProcessID, state: StateMachineNodeID, other_process: ProcessID, other_state: StateMachineNodeID) -> ProcessReachabilityPair {
        if process.gt(&other_process) || (process.eq(&other_process) && state.ge(&other_state)) {
            ProcessReachabilityPair(process, state, other_process, other_state)
        } else {
            ProcessReachabilityPair(other_process, other_state, process, state)
        }
    }

    pub fn get_left(&self) -> (ProcessID, StateMachineNodeID) {
        (self.0, self.1)
    }

    pub fn get_right(&self) -> (ProcessID, StateMachineNodeID) {
        (self.2, self.3)
    }

    pub fn get_processes(&self) -> (ProcessID, ProcessID) {
        return (self.0, self.2);
    }

    pub fn get_process_states(&self) -> (StateMachineNodeID, StateMachineNodeID) {
        return (self.1, self.3);
    }

    pub fn is_same_process(&self) -> bool {
        return self.0 == self.2;
    }

    pub fn product(&self, other: ProcessReachabilityPair) -> [ProcessReachabilityPair; 4] {
        return [
            ProcessReachabilityPair::new(self.0, self.1, other.0, other.1),
            ProcessReachabilityPair::new(self.0, self.1, other.2, other.3),
            ProcessReachabilityPair::new(self.2, self.3, other.0, other.1),
            ProcessReachabilityPair::new(self.2, self.3, other.2, other.3)
        ];
    }
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

    pub fn mark_cooccuring(&mut self, pair: &ProcessReachabilityPair) {
        self.mark_active(pair.get_left().0, pair.get_left().1);
        self.mark_active(pair.get_right().0, pair.get_right().1);

        self.cooccuring_state_pairs.entry(pair.get_processes())
            .or_default()
            .insert(pair.get_process_states());
        self.cooccuring_state_pairs.entry(rev2(pair.get_processes()))
            .or_default()
            .insert(rev2(pair.get_process_states()));
    }
}
