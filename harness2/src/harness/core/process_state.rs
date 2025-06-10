use std::{collections::{BTreeMap, BTreeSet, HashSet}, hash::Hash};

use super::{error::HarnessError, process::{ProcessID, ProcessSet}, reachability::{ProcessStateReachability, ProcessReachabilityPair}, state_machine::{StateMachineContext, StateMachineEdgeID, StateMachineMessageEnvelope, StateMachineMessageEnvelopeBehavior, StateMachineMessageID, StateMachineNodeID}};

#[derive(Clone, Debug, Eq)]
struct ProcessState {
    process_id: ProcessID,
    node: StateMachineNodeID,
    inbox: BTreeMap<ProcessID, BTreeSet<StateMachineMessageID>>,
}

#[derive(Clone, Debug, Eq)]
pub struct ProcessSetState {
    processes: BTreeMap<ProcessID, ProcessState>
}

pub struct ProcessSetStateSpace {
    states: HashSet<ProcessSetState>
}

impl Hash for ProcessState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.process_id.hash(state);
        self.node.hash(state);
        for (sender, message) in &self.inbox {
            sender.hash(state);
            message.hash(state);
        }
    }
}

impl PartialEq for ProcessState {
    fn eq(&self, other: &Self) -> bool {
        if self.process_id != other.process_id {
            return false;
        }

        if self.node != other.node {
            return false;
        }


        for (sender, message) in &self.inbox {
            match other.inbox.get(sender) {
                Some(msg) if msg == message => (),
                _ => return false
            }
        }

        for (sender, message) in &other.inbox {
            match self.inbox.get(sender) {
                Some(msg) if msg == message => (),
                _ => return false
            }
        }

        true
    }
}

impl ProcessState {
    fn next_transitions(&self, context: &StateMachineContext, process_set: &ProcessSet) -> Result<Vec<(ProcessState, StateMachineEdgeID, Vec<StateMachineMessageEnvelope>)>, HarnessError> {
        let mut transitions = Vec::new();
        let mut has_triggered_transitions = false;
        for (&origin, triggers) in self.inbox.iter() {
            for &trigger in triggers {
                let mut new_state = self.clone();
                new_state.inbox.remove(&origin);
                for transition in self.next_triggered_transitions(context, process_set, (origin, trigger), new_state)? {
                    transitions.push(transition);
                    has_triggered_transitions = true;
                }
            }
        }
        if !has_triggered_transitions {
            transitions.extend(self.next_untriggered_transitions(context, process_set)?);
        }
        Ok(transitions)
    }

    fn next_triggered_transitions(&self, context: &StateMachineContext, process_set: &ProcessSet, trigger: (ProcessID, StateMachineMessageID), state: ProcessState) -> Result<Vec<(ProcessState, StateMachineEdgeID, Vec<StateMachineMessageEnvelope>)>, HarnessError> {
        let mut transitions = Vec::new();
        let conditinal_edges = context.get_edges_from(state.node)
            .expect("Expected node to exist")  
            .filter(| edge | context.get_edge_trigger(*edge) == Some(trigger.1));
        for edge in conditinal_edges {
            transitions.push(self.next_transition_for_edge(context, process_set, state.clone(), edge, Some(trigger.0))?);
        }
        Ok(transitions)
    }

    fn next_untriggered_transitions(&self, context: &StateMachineContext, process_set: &ProcessSet) -> Result<Vec<(ProcessState, StateMachineEdgeID, Vec<StateMachineMessageEnvelope>)>, HarnessError> {
        let mut transitions = Vec::new();
        let unconditional_edges = context.get_edges_from(self.node)
            .expect("Expected node to exist")
            .filter(| edge | context.get_edge_trigger(*edge).is_none());
        for edge in unconditional_edges {
            transitions.push(self.next_transition_for_edge(context, process_set, self.clone(), edge, None)?);
        }
        Ok(transitions)
    }

    fn next_transition_for_edge(&self, context: &StateMachineContext, process_set: &ProcessSet, state: ProcessState, edge: StateMachineEdgeID, trigger_origin: Option<ProcessID>) -> Result<(ProcessState, StateMachineEdgeID, Vec<StateMachineMessageEnvelope>), HarnessError> {
        let action = context.get_edge_action(edge);
        let outbound_envelopes = match action {
            Some(action) => context.get_envelopes(action)
                .expect("Expected action to exist")
                .map(| envelope | process_set.map_outbound_message(self.process_id, trigger_origin, edge, envelope))
                .collect::<Result<Vec<StateMachineMessageEnvelope>, HarnessError>>()?,
            None => Vec::new()
        };
        let mut new_state = state;
        new_state.node = context.get_edge_target(edge).expect("Expected edge to exist");
        Ok((new_state, edge, outbound_envelopes))
    }

    fn with_empty_inbox(&self) -> Self {
        Self {
            process_id: self.process_id,
            node: self.node,
            inbox: BTreeMap::default()
        }
    }
}

impl Hash for ProcessSetState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for process_state in self.processes.values() {
            process_state.hash(state);
        }
    }
}

impl PartialEq for ProcessSetState {
    fn eq(&self, other: &Self) -> bool {
        for (process_id, process_state) in &self.processes {
            match other.processes.get(process_id) {
                Some(state) if state == process_state => (),
                _ => return false
            }
        }

        for (process_id, process_state) in &other.processes {
            match self.processes.get(process_id) {
                Some(state) if state == process_state => (),
                _ => return false
            }
        }

        true
    }
}

impl ProcessSetState {
    pub fn new(process_set: &ProcessSet) -> ProcessSetState {
        let processes = process_set.get_processes()
            .map(| process_id| (process_id, ProcessState {
                process_id,
                node: process_set.get_process_entry_node(process_id).expect("Expected process to exist"),
                inbox: BTreeMap::new()
            }))
            .collect();
        ProcessSetState {
            processes
        }
    }

    pub fn new_from(processes: impl Iterator<Item = (ProcessID, StateMachineNodeID)>) -> ProcessSetState {
        ProcessSetState {
            processes: processes.map(| (process, state) | (process, ProcessState {
                process_id: process,
                node: state,
                inbox: BTreeMap::default()
            })).collect()
        }
    }

    pub fn get_process_node(&self, process_id: ProcessID) -> Option<StateMachineNodeID> {
        self.processes.get(&process_id)
            .map(| state | state.node)
    }

    pub fn get_process_inbox(&self, process_id: ProcessID) -> Option<impl Iterator<Item = (ProcessID, StateMachineMessageID)>> {
        self.processes.get(&process_id)
            .map(| process | process.inbox.iter()
                .map(| (&sender, msgs) | msgs.iter().map(move | &msg | (sender, msg))).flatten())
    }

    pub fn get_next_transitions(&self, context: &StateMachineContext, process_set: &ProcessSet) -> Result<Vec<(ProcessSetState, ProcessID, StateMachineEdgeID)>, HarnessError> {
        let mut transitions = Vec::new();

        for process_state in self.processes.values() {
            for transition in self.next_transitions_for(context, process_set, process_state)? {
                transitions.push(transition);
            }
        }

        Ok(transitions)
    }

    pub fn with_empty_inboxes(&self) -> Self {
        Self {
            processes: self.processes.iter().map(| (process, state) | (*process, state.with_empty_inbox())).collect()
        }
    }

    fn next_transitions_for(&self, context: &StateMachineContext, process_set: &ProcessSet, process_state: &ProcessState) -> Result<Vec<(ProcessSetState, ProcessID, StateMachineEdgeID)>, HarnessError> {
        let mut transitions = Vec::new();
        for (next_state, transition_edge, outbound_envelopes) in process_state.next_transitions(context, process_set)? {
            let mut new_process_set_state = self.clone();
            new_process_set_state.processes.insert(next_state.process_id, next_state);
            let mut ignore_transition = false;
            for envelope in &outbound_envelopes {
                let matching_destinations = self.processes.keys()
                    .map(| process_id | *process_id)
                    .filter_map(| process_id | match envelope.get_destination().matches(process_id.into()) {
                        Ok(true) => Some(Ok(process_id)),
                        Ok(false) => None,
                        Err(err) => Some(Err(err))
                    })
                    .collect::<Result<Vec<ProcessID>, HarnessError>>()?;
                for receiver_id in matching_destinations {
                    let message = process_set.map_inbound_message(receiver_id, process_state.process_id, envelope.get_message())?;
                    let mailbox = new_process_set_state.processes.get_mut(&receiver_id)
                        .ok_or(HarnessError::new("Unable to find receiver process for a message"))?
                        .inbox.entry(process_state.process_id)
                        .or_insert(BTreeSet::new());

                    match envelope.get_behavior() {
                        StateMachineMessageEnvelopeBehavior::BlockAnyMessage if !mailbox.is_empty() => ignore_transition = true,
                        StateMachineMessageEnvelopeBehavior::BlockSameMessage if mailbox.contains(&message) => ignore_transition = true,
                        StateMachineMessageEnvelopeBehavior::ReplaceAnyMessage => mailbox.clear(),
                        _ => {}
                    }
                    
                    if !ignore_transition {
                        mailbox.insert(message);
                    } else {
                        break;
                    }
                }

                if ignore_transition {
                    break;
                }
            }

            if !ignore_transition {
                transitions.push((new_process_set_state, process_state.process_id, transition_edge));
            }
        }
        Ok(transitions)
    }

    pub fn get_reachable_state_space(&self, context: &StateMachineContext, process_set: &ProcessSet) -> Result<ProcessSetStateSpace, HarnessError> {
        let mut space = HashSet::new();
        let mut queue = Vec::from([self.clone()]);

        while !queue.is_empty() {
            let state = queue.pop().expect("Expected queue to be non-empty");
            if space.contains(&state) {
                continue;
            }

            queue.extend(state.get_next_transitions(context, process_set)?
                .into_iter()
                .map(| (state, _, _) | state));

            space.insert(state);
        }

        Ok(ProcessSetStateSpace {
            states: space
        })
    }
}

impl ProcessSetStateSpace {
    pub fn new(states: impl Iterator<Item = ProcessSetState>) -> ProcessSetStateSpace {
        ProcessSetStateSpace { states: states.collect() }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ProcessSetState> {
        self.states.iter()
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn derive_reachability(&self) -> ProcessStateReachability {
        let mut reachability = ProcessStateReachability::new();

        for psstate in self.iter() {
            for (&process, process_state) in &psstate.processes {
                reachability.mark_active(process, process_state.node);

                for (&other_process, other_process_state) in &psstate.processes {
                    if other_process != process {
                        reachability.mark_cooccuring(&ProcessReachabilityPair::new(process, process_state.node, other_process, other_process_state.node));
                    }
                }
            }
        }

        reachability
    }
}
