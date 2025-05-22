use std::{collections::{HashMap, HashSet}, rc::Rc};

use crate::harness::core::{error::HarnessError, process::{ProcessID, ProcessSet}, state_machine::{StateMachineContext, StateMachineEdgeID, StateMachineNodeID}};

use super::{mutex::ControlFlowMutexSet, node::{ControlFlowLabel, ControlFlowLabelGenerator, ControlFlowNode}};

pub struct ControlFlowBuilder {
    backward_edges: HashSet<StateMachineEdgeID>,
    labelled_states: HashMap<StateMachineNodeID, ControlFlowLabel>
}

impl ControlFlowBuilder {
    pub fn new(context: &StateMachineContext, root: StateMachineNodeID) -> Result<ControlFlowBuilder, HarnessError> {
        let mut builder = ControlFlowBuilder {
            backward_edges: HashSet::new(),
            labelled_states: HashMap::new()
        };
        builder.initialize(context, root)?;
        Ok(builder)
    }

    pub fn build(&self, context: &StateMachineContext, process_set: &ProcessSet, process: ProcessID, mutex_set: &ControlFlowMutexSet<'_>) -> Result<ControlFlowNode, HarnessError> {
        let process_entry_node = process_set.get_process_entry_node(process)
            .ok_or(HarnessError::new("Unable to find process entry node"))?;
        let locked_initially = mutex_set.get_locked_mutexes_in_state(process, process_entry_node)
            .collect::<HashSet<_>>();
        let node = self.generate_control_flow_node(context, process, process_entry_node, mutex_set, &mut HashSet::new())?;
        if locked_initially.is_empty() {
            Ok(ControlFlowNode::Sequence(Vec::from([
                ControlFlowNode::InitBarrier,
                node
            ])))
        } else {
            Ok(ControlFlowNode::Sequence(Vec::from([
                ControlFlowNode::Synchronization(locked_initially, HashSet::new(), None),
                ControlFlowNode::InitBarrier,
                node
            ])))
        }
    }

    fn initialize(&mut self, context: &StateMachineContext, root: StateMachineNodeID) -> Result<(), HarnessError> {
        let mut label_gen = ControlFlowLabelGenerator::new();
        let mut visited = HashSet::new();
        let mut queue = Vec::from([(root, Rc::new(Vec::new()))]);
        while !queue.is_empty() {
            let (current_state, current_path) = queue.pop().expect("Expected queue to be non-empty");
            if visited.contains(&current_state) {
                continue;
            }
            visited.insert(current_state);

            if !self.labelled_states.contains_key(&current_state) {
                self.labelled_states.insert(current_state, label_gen.next_label());
            }

            let new_path = Rc::new({
                let mut path = current_path.as_ref().clone();
                path.push(current_state);
                path
            });

            let edges = context.get_edges_from(current_state)
                .ok_or(HarnessError::new("Unable to retrieve edges coming from a state"))?;
            for edge in edges {
                let edge_target = context.get_edge_target(edge)
                    .ok_or(HarnessError::new("Unable to retrieve edge target"))?;
                if new_path.contains(&edge_target) {
                    self.backward_edges.insert(edge);
                } else {
                    queue.push((edge_target, new_path.clone()));
                }
            }
        }
        Ok(())
    }

    fn generate_control_flow_node(&self, context: &StateMachineContext, process: ProcessID, state: StateMachineNodeID, mutex_set: &ControlFlowMutexSet<'_>, generated_states: &mut HashSet<StateMachineNodeID>) -> Result<ControlFlowNode, HarnessError> {
        generated_states.insert(state);

        let edges = context.get_edges_from(state)
            .ok_or(HarnessError::new("Unable to retrieve edges originating from a state"))?;
        let mut edge_nodes = edges
            .map(| edge | self.generate_edge(context, process, edge, mutex_set, generated_states))
            .collect::<Result<Vec<_>, HarnessError>>()?;

        let node = if edge_nodes.len() > 1 {
            ControlFlowNode::Branch(edge_nodes)
        } else if !edge_nodes.is_empty() {
            edge_nodes.pop().expect("Expected control-flow node list to be non-empty")
        } else {
            ControlFlowNode::Sequence(Vec::new())
        };

        match self.labelled_states.get(&state) {
            Some(label) =>
                Ok(ControlFlowNode::LabelledNode(*label, node.into())),
            _ => Ok(node)
        }
    }

    fn generate_edge(&self, context: &StateMachineContext, process: ProcessID, edge: StateMachineEdgeID, mutex_set: &ControlFlowMutexSet<'_>, generated_states: &mut HashSet<StateMachineNodeID>) -> Result<ControlFlowNode, HarnessError> {
        let edge_source = context.get_edge_source(edge)
            .ok_or(HarnessError::new("Unable to retrieve edge source"))?;
        let edge_target = context.get_edge_target(edge)
            .ok_or(HarnessError::new("Unable to retrieve edge target"))?;

        let mut sequence = if self.backward_edges.contains(&edge) || generated_states.contains(&edge_target) {
            let label = *self.labelled_states.get(&edge_target).expect("Expected edge target state to have a label");
            Vec::from([
                ControlFlowNode::Statement(edge),
                ControlFlowNode::Goto(label)
            ])
        } else {
            Vec::from([
                ControlFlowNode::Statement(edge),
                self.generate_control_flow_node(context, process, edge_target, mutex_set, generated_states)?
            ])
        };

        let mut locked_mutexes = mutex_set.get_locked_mutexes_in_state(process, edge_target)
            .collect::<HashSet<_>>();
        let mut unlocked_mutexes = mutex_set.get_locked_mutexes_in_state(process, edge_source)
            .collect::<HashSet<_>>();

        let hold_mutexes = locked_mutexes.intersection(&unlocked_mutexes)
            .map(| x | *x)
            .collect::<HashSet<_>>();

        unlocked_mutexes = unlocked_mutexes.difference(&hold_mutexes)
            .map(| x | *x)
            .collect();
        locked_mutexes = locked_mutexes.difference(&hold_mutexes)
            .map(| x | *x)
            .collect();

        if !locked_mutexes.is_empty() || !unlocked_mutexes.is_empty() {
            let label = *self.labelled_states.get(&edge_source).expect("Expected edge target state to have a label");
            sequence.insert(0, ControlFlowNode::Synchronization(locked_mutexes, unlocked_mutexes, Some(label)));
        }
        Ok(ControlFlowNode::Sequence(sequence))
    }
}
