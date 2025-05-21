use std::collections::HashSet;

use crate::harness::core::state_machine::StateMachineEdgeID;

use super::mutex::ControlFlowMutexID;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ControlFlowLabel(u64);

pub struct ControlFlowLabelGenerator(u64);

#[derive(Debug, Clone)]
pub enum ControlFlowNode {
    Statement(StateMachineEdgeID),
    Sequence(Vec<ControlFlowNode>),
    Branch(Vec<ControlFlowNode>),
    Synchronization(HashSet<ControlFlowMutexID>, HashSet<ControlFlowMutexID>, Option<ControlFlowLabel>),
    LabelledNode(ControlFlowLabel, Box<ControlFlowNode>),
    Goto(ControlFlowLabel),
    InitBarrier
}

impl Into<u64> for ControlFlowLabel {
    fn into(self) -> u64 {
        self.0
    }
}

impl ControlFlowLabelGenerator {
    pub fn new() -> ControlFlowLabelGenerator {
        ControlFlowLabelGenerator(0)
    }

    pub fn next_label(&mut self) -> ControlFlowLabel {
        let label = ControlFlowLabel(self.0);
        self.0 += 1;
        label
    }
}

impl ControlFlowNode {
    pub fn canonicalize(&self) -> ControlFlowNode {
        match self {
            Self::Sequence(elts) => {
                let canonicalized_elts = elts.iter().map(| node | node.canonicalize());
                let mut new_sequence = Vec::new();
                for node in canonicalized_elts {
                    if let Self::Sequence(subseq) = node {
                        new_sequence.extend(subseq);
                    } else {
                        new_sequence.push(node);
                    }
                }
                if new_sequence.len() == 1 {
                    return new_sequence.pop().unwrap();
                } else {
                    Self::Sequence(new_sequence)
                }
            },
            Self::Branch(elts) =>
                Self::Branch(elts.iter().map(| node | node.canonicalize()).collect()),
            Self::LabelledNode(label, body) =>
                Self::LabelledNode(*label, body.canonicalize().into()),
            _ => self.clone()
        }
    }
}