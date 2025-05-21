use std::{collections::BTreeSet, hash::Hash};

use crate::harness::core::{process::ProcessID, state_machine::StateMachineNodeID};

#[derive(Debug, Clone, Eq)]
pub struct MutualExclusionSegment(BTreeSet<(ProcessID, StateMachineNodeID)>);

impl PartialEq for MutualExclusionSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0.symmetric_difference(&other.0).count() == 0
    }
}

impl Hash for MutualExclusionSegment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (process, node) in &self.0 {
            process.hash(state);
            node.hash(state);
        }
    }
}

impl Default for MutualExclusionSegment {
    fn default() -> Self {
        Self::empty()
    }
}

impl Into<MutualExclusionSegment> for BTreeSet<(ProcessID, StateMachineNodeID)> {
    fn into(self) -> MutualExclusionSegment {
        MutualExclusionSegment(self)
    }
}

impl MutualExclusionSegment {
    pub fn empty() -> Self {
        MutualExclusionSegment(BTreeSet::new())
    }

    pub fn get_processes(&self) -> impl Iterator<Item = ProcessID> {
        self.0.iter().map(| (process, _) | *process)
    }

    pub fn iter(&self) -> impl Iterator<Item = (ProcessID, StateMachineNodeID)> {
        self.0.iter().map(| (process, state) | (*process, *state))
    }

    pub fn has(&self, process: ProcessID, state: StateMachineNodeID) -> bool {
        self.0.contains(&(process, state))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn extend(&self, process: ProcessID, state: StateMachineNodeID) -> MutualExclusionSegment {
        let mut content = self.0.clone();
        content.insert((process, state));
        MutualExclusionSegment(content)
    }

    pub fn union(&self, segment: impl Iterator<Item = (ProcessID, StateMachineNodeID)>) -> MutualExclusionSegment {
        let mut content = self.0.clone();
        content.extend(segment);
        MutualExclusionSegment(content)
    }

    pub fn intersection(&self, segment: impl Iterator<Item = (ProcessID, StateMachineNodeID)>) -> MutualExclusionSegment {
        MutualExclusionSegment(self.0.intersection(&segment.collect()).map(| x | *x).collect())
    }

    pub fn difference(&self, segment: impl Iterator<Item = (ProcessID, StateMachineNodeID)>) -> MutualExclusionSegment {
        MutualExclusionSegment(self.0.difference(&segment.collect()).map(| x | *x).collect())
    }

    pub fn includes(&self, segment: impl Iterator<Item = (ProcessID, StateMachineNodeID)>) -> bool {
        for (process, state) in segment {
            if !self.has(process, state) {
                return false;
            }
        }
        true
    }
}
