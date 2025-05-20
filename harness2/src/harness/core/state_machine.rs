use std::{cell::RefCell, collections::{HashMap, HashSet}};

use super::error::HarnessError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StateMachineMessageID(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StateMachineNodeID(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StateMachineActionID(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StateMachineEdgeID(u64);


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StateMachineMessageParticipantID(pub u64);

#[derive(Debug, Clone)]
pub enum StateMachineMessageDestination {
    Unicast(StateMachineMessageParticipantID),
    Multicast(HashSet<StateMachineMessageParticipantID>),
    Response
}

#[derive(Debug, Clone)]
pub struct StateMachineMessageEnvelope {
    destination: StateMachineMessageDestination,
    message: StateMachineMessageID
}

pub struct EdgeData {
    source: StateMachineNodeID,
    target: StateMachineNodeID,
    trigger: Option<StateMachineMessageID>,
    action: Option<StateMachineActionID>
}

struct EntityData {
    mnemonic: String
}

struct ActionData {
    mnemonic: String,
    envelopes: Vec<StateMachineMessageEnvelope>
}

pub struct StateMachineContext {
    messages: HashMap<StateMachineMessageID, EntityData>,
    nodes: HashMap<StateMachineNodeID, EntityData>,
    actions: HashMap<StateMachineActionID, ActionData>,
    edges: HashMap<StateMachineEdgeID, EdgeData>,
    forward_edges: HashMap<StateMachineNodeID, HashSet<StateMachineEdgeID>>,
    reverse_edges: HashMap<StateMachineNodeID, HashSet<StateMachineEdgeID>>,
    reachable_nodes: RefCell<HashMap<StateMachineNodeID, HashSet<StateMachineNodeID>>>
}

impl From<String> for EntityData {
    fn from(value: String) -> Self {
        EntityData { mnemonic: value }
    }
}

impl From<String> for ActionData {
    fn from(value: String) -> Self {
        ActionData { mnemonic: value, envelopes: Vec::new() }
    }
}

impl StateMachineMessageDestination {
    pub fn matches(&self, participant: StateMachineMessageParticipantID) -> Result<bool, HarnessError> {
        match self {
            Self::Unicast(part) if *part == participant => Ok(true),
            Self::Multicast(parts) if parts.contains(&participant) => Ok(true),
            Self::Response => Err(HarnessError("TODO".into())),
            _ => Ok(false)
        }
    }
}

impl StateMachineMessageEnvelope {
    pub fn get_destination(&self) -> &StateMachineMessageDestination {
        &self.destination
    }

    pub fn get_message(&self) -> StateMachineMessageID {
        self.message
    }

    pub fn redirect(&self, destination: StateMachineMessageDestination) -> StateMachineMessageEnvelope {
        StateMachineMessageEnvelope {
            destination,
            message: self.message
        }
    }
}

impl<'a> StateMachineContext {
    pub fn new() -> StateMachineContext {
        StateMachineContext {
            messages: HashMap::new(),
            nodes: HashMap::new(),
            actions: HashMap::new(),
            edges: HashMap::new(),
            forward_edges: HashMap::new(),
            reverse_edges: HashMap::new(),
            reachable_nodes: RefCell::new(HashMap::new())
        }
    }

    pub fn new_message(&mut self, mnemonic: String) -> Result<StateMachineMessageID, HarnessError> {
        let msg = StateMachineMessageID(self.messages.len() as u64);
        self.messages.insert(msg, mnemonic.into());
        Ok(msg)
    }

    pub fn new_node(&mut self, mnemonic: String) -> Result<StateMachineNodeID, HarnessError> {
        let node = StateMachineNodeID(self.nodes.len() as u64);
        self.nodes.insert(node, mnemonic.into());
        self.forward_edges.insert(node, HashSet::new());
        self.reverse_edges.insert(node, HashSet::new());
        Ok(node)
    }

    pub fn new_action(&mut self, mnemonic: String) -> Result<StateMachineActionID, HarnessError> {
        let action = StateMachineActionID(self.actions.len() as u64);
        self.actions.insert(action, mnemonic.into());
        Ok(action)
    }

    pub fn new_edge(&mut self, source: StateMachineNodeID, target: StateMachineNodeID, trigger: Option<StateMachineMessageID>, action: Option<StateMachineActionID>) -> Result<StateMachineEdgeID, HarnessError> {
        if !self.nodes.contains_key(&source) || !self.nodes.contains_key(&target) {
            return Err(HarnessError("TODO".into()));
        }

        match trigger {
            Some(message) if !self.messages.contains_key(&message) =>
                return Err(HarnessError("TODO".into())),
            _ => ()
        };
        match action {
            Some(action) if !self.actions.contains_key(&action) =>
                return Err(HarnessError("TODO".into())),
            _ => ()
        }

        let edge_id = StateMachineEdgeID(self.edges.len() as u64);
        let edge = EdgeData {
            source,
            target,
            trigger,
            action
        };

        self.edges.insert(edge_id, edge);
        self.forward_edges.get_mut(&source)
            .ok_or(HarnessError("TODO".into()))?
            .insert(edge_id);
        self.reverse_edges.get_mut(&target)
            .ok_or(HarnessError("TODO".into()))?
            .insert(edge_id);
        Ok(edge_id)
    }

    pub fn add_envelope(&mut self, action: StateMachineActionID, destination: StateMachineMessageDestination, message: StateMachineMessageID) -> Result<(), HarnessError> {
        match self.actions.get_mut(&action) {
            Some(action_data) => {
                action_data.envelopes.push(StateMachineMessageEnvelope {
                    destination,
                    message
                });
                Ok(())
            },
            None => Err(HarnessError("TODO".into()))
        }
    }

    pub fn get_message_mnemonic(&self, message: StateMachineMessageID) -> Option<&str> {
        self.messages.get(&message).map(| data | data.mnemonic.as_str())
    }

    pub fn get_node_mnemonic(&self, node: StateMachineNodeID) -> Option<&str> {
        self.nodes.get(&node).map(| data | data.mnemonic.as_str())
    }

    pub fn get_action_mnemonic(&self, action: StateMachineActionID) -> Option<&str> {
        self.actions.get(&action).map(| data | data.mnemonic.as_str())
    }

    pub fn get_edge_source(&self, edge_id: StateMachineEdgeID) -> Option<StateMachineNodeID> {
        self.edges.get(&edge_id).map(| edge | edge.source)
    }

    pub fn get_edge_target(&self, edge_id: StateMachineEdgeID) -> Option<StateMachineNodeID> {
        self.edges.get(&edge_id).map(| edge | edge.target)
    }

    pub fn get_edge_trigger(&self, edge_id: StateMachineEdgeID) -> Option<StateMachineMessageID> {
        self.edges.get(&edge_id).map(| edge | edge.trigger).flatten()
    }

    pub fn get_edge_action(&self, edge_id: StateMachineEdgeID) -> Option<StateMachineActionID> {
        self.edges.get(&edge_id).map(| edge | edge.action).flatten()
    }

    pub fn get_edges_from(&self, source: StateMachineNodeID) -> Option<impl Iterator<Item = StateMachineEdgeID>> {
        self.forward_edges.get(&source)
            .map(| edges | edges.iter().map(| x | *x))
    }

    pub fn get_edges_to(&self, target: StateMachineNodeID) -> Option<impl Iterator<Item = StateMachineEdgeID>> {
        self.reverse_edges.get(&target)
            .map(| edges | edges.iter().map(| x | *x))
    }

    pub fn get_envelopes(&self, action: StateMachineActionID) -> Option<impl Iterator<Item = &StateMachineMessageEnvelope>> {
        self.actions.get(&action)
            .map(| action_data | action_data.envelopes.iter())
    }

    pub fn get_nodes_reachable_from(&self, root: StateMachineNodeID) -> Result<Vec<StateMachineNodeID>, HarnessError> {
        match self.reachable_nodes.borrow().get(&root) {
            Some(nodes) =>
                return Ok(nodes.iter().map(| x | *x).collect()),
            _ => ()
        };

        if !self.nodes.contains_key(&root) {
            return Err(HarnessError("TODO".into()));
        }

        let mut reachable = HashSet::new();
        let mut queue = Vec::from([root]);
        while !queue.is_empty() {
            let node = queue.pop().expect("Expected non-empty queue");
            if reachable.contains(&node) {
                continue;
            }
            reachable.insert(node);

            for edge in self.get_edges_from(node).expect("Expected node to exist") {
                queue.push(self.get_edge_target(edge).expect("Expected to find edge"));
            }
        }

        self.reachable_nodes.borrow_mut().insert(root, reachable);
        Ok(self.reachable_nodes.borrow().get(&root).unwrap().iter().map(| x | *x).collect())
    }
}
