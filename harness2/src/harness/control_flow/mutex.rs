use std::collections::HashMap;

use crate::harness::core::{mutex::segment::MutualExclusionSegment, process::ProcessID, state_machine::StateMachineNodeID};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ControlFlowMutexID(u64);

pub struct ControlFlowMutex<'a>(ControlFlowMutexID, &'a MutualExclusionSegment);

pub struct ControlFlowMutexSet<'a> {
    mutexes: HashMap<ControlFlowMutexID, ControlFlowMutex<'a>>,
    segment_mutexes: HashMap<&'a MutualExclusionSegment, ControlFlowMutexID>
}

impl Into<u64> for ControlFlowMutexID {
    fn into(self) -> u64 {
        self.0
    }
}

impl<'a> ControlFlowMutex<'a> {
    pub fn get_identifier(&self) -> ControlFlowMutexID {
        self.0
    }

    pub fn get_segment(&self) -> &'a MutualExclusionSegment {
        self.1
    }
}

impl<'a> ControlFlowMutexSet<'a> {
    pub fn new(segments: impl Iterator<Item = &'a MutualExclusionSegment>) -> ControlFlowMutexSet<'a> {
        let mut mutexes = HashMap::new();
        let mut segment_mutexes = HashMap::new();
        for segment in segments {
            if !segment_mutexes.contains_key(segment) {
                let mutex_id = ControlFlowMutexID(mutexes.len() as u64);
                mutexes.insert(mutex_id, ControlFlowMutex(mutex_id, segment));
                segment_mutexes.insert(segment, mutex_id);
            }
        }
        ControlFlowMutexSet {
            mutexes,
            segment_mutexes
        }
    }

    pub fn get_mutex(&self, mutex_id: ControlFlowMutexID) -> Option<&ControlFlowMutex<'a>> {
        self.mutexes.get(&mutex_id)
    }

    pub fn get_mutexes(&self) -> impl Iterator<Item = &ControlFlowMutex<'a>> {
        self.mutexes.values()
    }

    pub fn get_mutex_for(&self, segment: &'_ MutualExclusionSegment) -> Option<ControlFlowMutexID> {
        self.segment_mutexes.get(segment).map(| mutex | *mutex)
    }

    pub fn get_locked_mutexes_in_state(&self, process: ProcessID, state: StateMachineNodeID) -> impl Iterator<Item = ControlFlowMutexID> {
        self.mutexes.values()
            .flat_map(move | mutex | {
                if mutex.get_segment().has(process, state) {
                    Some(mutex.get_identifier())
                } else {
                    None
                }
            })
    }
}
