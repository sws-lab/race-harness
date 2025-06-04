use std::{collections::{BTreeMap, HashMap, HashSet}, hash::Hash};

use crate::harness::{core::{error::HarnessError, process::{ProcessID, ProcessSet}, state_machine::{StateMachineActionID, StateMachineContext, StateMachineMessageDestination, StateMachineMessageEnvelopeBehavior, StateMachineMessageID, StateMachineMessageParticipantID, StateMachineNodeID}}, entities::product_node::{StateMachineProductNode, StateMachineProductNodeBuilder}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct HarnessModelSymbolID(u64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HarnessModelSymbol {
    State(HarnessModelSymbolID),
    Edge(HarnessModelSymbolID),
    Message(HarnessModelSymbolID),
    Action(HarnessModelSymbolID),
    Process(HarnessModelSymbolID)
}

#[derive(Clone)]
enum HarnessModelStateBuilder {
    Primitive {
        mnemonic: String
    },

    Product {
        base_state: HarnessModelSymbol,
        mapped_processes: Vec<HarnessModelSymbol>
    },

    ProductElement {
        product_state: HarnessModelSymbol,
        element_structure: Vec<HarnessModelSymbol>
    }
}

#[derive(Clone)]
struct HarnessMessageBuilder {
    mnemonic: String
}

#[derive(Clone)]
struct HarnessEdgeBuilder {
    target: HarnessModelSymbol,
    trigger: Option<HarnessModelSymbol>,
    action: Option<HarnessModelSymbol>
}

#[derive(Clone)]
enum HarnessEnvelopeBuilder {
    Unicast(HarnessModelSymbol, StateMachineMessageEnvelopeBehavior, HarnessModelSymbol),
    Multicast(Vec<HarnessModelSymbol>, StateMachineMessageEnvelopeBehavior, HarnessModelSymbol),
    Response(StateMachineMessageEnvelopeBehavior, HarnessModelSymbol)
}

#[derive(Clone)]
struct HarnessActionBuilder {
    mnemonic: String,
    envelopes: Vec<HarnessEnvelopeBuilder>
}

#[derive(Clone)]
struct HarnessProcessBuilder {
    mnemonic: String,
    entry_state: HarnessModelSymbol
}

pub struct HarnessModelBuild {
    states: HashMap<HarnessModelSymbol, StateMachineNodeID>,
    messages: HashMap<HarnessModelSymbol, StateMachineMessageID>,
    actions: HashMap<HarnessModelSymbol, StateMachineActionID>,
    processes: HashMap<HarnessModelSymbol, ProcessID>,
    product_nodes: HashMap<HarnessModelSymbol, StateMachineProductNode>,
    pending_product_mappings: Vec<(HarnessModelSymbol, HarnessModelSymbol, Vec<HarnessModelSymbol>)>
}

#[derive(Clone)]
pub struct HarnessModel {
    next_symbol: u64,
    named_symbols: HashMap<String, HarnessModelSymbol>,
    states: HashMap<HarnessModelSymbol, HarnessModelStateBuilder>,
    messages: HashMap<HarnessModelSymbol, HarnessMessageBuilder>,
    edges: HashMap<HarnessModelSymbol, HarnessEdgeBuilder>,
    direct_edges: HashMap<HarnessModelSymbol, Vec<HarnessModelSymbol>>,
    actions: HashMap<HarnessModelSymbol, HarnessActionBuilder>,
    processes: BTreeMap<HarnessModelSymbol, HarnessProcessBuilder>,
    product_subnodes: HashMap<HarnessModelSymbol, HashSet<HarnessModelSymbol>>
}

impl HarnessModelBuild {
    pub fn get_state(&self, symbol: HarnessModelSymbol) -> Option<StateMachineNodeID> {
        self.states.get(&symbol).map(| x | *x)
    }

    pub fn get_message(&self, symbol: HarnessModelSymbol) -> Option<StateMachineMessageID> {
        self.messages.get(&symbol).map(| x | *x)
    }

    pub fn get_action(&self, symbol: HarnessModelSymbol) -> Option<StateMachineActionID> {
        self.actions.get(&symbol).map(| x | *x)
    }

    pub fn get_process(&self, symbol: HarnessModelSymbol) -> Option<ProcessID> {
        self.processes.get(&symbol).map(| x | *x)
    }
}

impl HarnessModel {
    pub fn new() -> HarnessModel {
        HarnessModel {
            next_symbol: 0,
            named_symbols: HashMap::new(),
            states: HashMap::new(),
            messages: HashMap::new(),
            edges: HashMap::new(),
            direct_edges: HashMap::new(),
            actions: HashMap::new(),
            processes: BTreeMap::new(),
            product_subnodes: HashMap::new()
        }
    }

    pub fn new_primitive_state(&mut self, mnemonic: &str) -> Result<HarnessModelSymbol, HarnessError> {
        let symbol = self.new_named_state_symbol(mnemonic)?;
        self.states.entry(symbol)
            .or_insert(HarnessModelStateBuilder::Primitive { mnemonic: mnemonic.into() });
        Ok(symbol)
    }

    pub fn new_product_state(&mut self, mnemonic: &str, base_state: HarnessModelSymbol, mapped_processes: impl Iterator<Item = HarnessModelSymbol>) -> Result<HarnessModelSymbol, HarnessError> {
        let symbol = self.new_named_state_symbol(mnemonic)?;
        if !self.states.contains_key(&symbol) {
            if !self.states.contains_key(&base_state) {
                return Err(HarnessError::new("Unknown product state base state symbol"));
            }
            let mapped_processes = mapped_processes.collect::<Vec<_>>();
            if mapped_processes.iter().any(| process | !self.processes.contains_key(&process)) {
                return Err(HarnessError::new("Unknown product state mapped process symbol"));
            }

            self.states.insert(symbol, HarnessModelStateBuilder::Product { base_state, mapped_processes });
        }
        
        Ok(symbol)
    }

    pub fn new_product_element_state<T: Into<Vec<HarnessModelSymbol>>>(&mut self, product_state: HarnessModelSymbol, element_structure: T) -> Result<HarnessModelSymbol, HarnessError> {
        if !self.states.contains_key(&product_state) {
            return Err(HarnessError::new("Unknown product state symbol"));
        }

        let structure: Vec<_> = element_structure.into();
        if structure.iter().any(| symbol | !self.states.contains_key(symbol)) {
            return Err(HarnessError::new("Unknown product substate structure symbol"));
        }

        let symbol = HarnessModelSymbol::State(self.new_symbol_id());
        self.states.insert(symbol, HarnessModelStateBuilder::ProductElement { product_state, element_structure: structure });
        self.product_subnodes.entry(product_state)
            .or_default()
            .insert(symbol);
        Ok(symbol)
    }

    pub fn new_message(&mut self, mnemonic: &str) -> Result<HarnessModelSymbol, HarnessError> {
        let symbol = self.new_named_message_symbol(mnemonic)?;
        self.messages.entry(symbol)
            .or_insert(HarnessMessageBuilder { mnemonic: mnemonic.into() });
        Ok(symbol)
    }

    pub fn new_action(&mut self, mnemonic: &str) -> Result<HarnessModelSymbol, HarnessError> {
        let symbol = self.new_named_action_symbol(mnemonic)?;
        self.actions.entry(symbol)
            .or_insert(HarnessActionBuilder { mnemonic: mnemonic.into(), envelopes: Vec::new() });
        Ok(symbol)
    }

    pub fn new_process(&mut self, mnemonic: &str, entry_state: HarnessModelSymbol) -> Result<HarnessModelSymbol, HarnessError> {
        if !self.states.contains_key(&entry_state) {
            return Err(HarnessError::new("Unknown process entry state symbol"));
        }
        let symbol = self.new_named_process_symbol(mnemonic)?;
        self.processes.entry(symbol)
            .or_insert(HarnessProcessBuilder { mnemonic: mnemonic.into(), entry_state });
        Ok(symbol)
    }

    pub fn new_edge(&mut self, source: HarnessModelSymbol, target: HarnessModelSymbol, trigger: Option<HarnessModelSymbol>, action: Option<HarnessModelSymbol>) -> Result<HarnessModelSymbol, HarnessError> {
        if !self.states.contains_key(&source) {
            return Err(HarnessError::new("Unknown edge source state symbol"));
        }
        if !self.states.contains_key(&target) {
            return Err(HarnessError::new("Unknown edge target state symbol"));
        }
        if let Some(trigger) = trigger {
            if !self.messages.contains_key(&trigger) {
                return Err(HarnessError::new("Unknown edge trigger message symbol"));
            }
        }
        if let Some(action) = action {
            if !self.actions.contains_key(&action) {
                return Err(HarnessError::new("Unknown edge action symbol"));
            }
        }

        let symbol = HarnessModelSymbol::Edge(self.new_symbol_id());
        self.edges.insert(symbol, HarnessEdgeBuilder { target, trigger, action });
        self.direct_edges.entry(source)
            .or_insert(Vec::new())
            .push(symbol);
        Ok(symbol)
    }

    pub fn new_unicast_envelope(&mut self, action: HarnessModelSymbol, destination: HarnessModelSymbol, behavior: StateMachineMessageEnvelopeBehavior, message: HarnessModelSymbol) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        if !self.processes.contains_key(&destination) {
            return Err(HarnessError::new("Unknown envelope destination process symbol"));
        }
        if !self.messages.contains_key(&message) {
            return Err(HarnessError::new("Unknown envelope message symbol"));
        }

        action.envelopes.push(HarnessEnvelopeBuilder::Unicast(destination, behavior, message));
        Ok(())
    }

    pub fn new_multicast_envelope(&mut self, action: HarnessModelSymbol, destinations: impl Iterator<Item = HarnessModelSymbol>, behavior: StateMachineMessageEnvelopeBehavior, message: HarnessModelSymbol) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        let destinations = destinations.collect::<Vec<_>>();
        if destinations.iter().any(| process | !self.processes.contains_key(&process)) {
            return Err(HarnessError::new("Unknown envelope destination process symbol"));
        }
        if !self.messages.contains_key(&message) {
            return Err(HarnessError::new("Unknown envelope message symbol"));
        }

        action.envelopes.push(HarnessEnvelopeBuilder::Multicast(destinations, behavior, message));
        Ok(())
    }

    pub fn new_response_envelope(&mut self, action: HarnessModelSymbol, behavior: StateMachineMessageEnvelopeBehavior, message: HarnessModelSymbol) -> Result<(), HarnessError> {
        let action = match self.actions.get_mut(&action) {
            Some(action) => action,
            None => return Err(HarnessError::new("Unknown action symbol"))
        };
        if !self.messages.contains_key(&message) {
            return Err(HarnessError::new("Unknown envelope message symbol"));
        }

        action.envelopes.push(HarnessEnvelopeBuilder::Response(behavior, message));
        Ok(())
    }

    fn new_symbol_id(&mut self) -> HarnessModelSymbolID {
        let symbol = HarnessModelSymbolID(self.next_symbol);
        self.next_symbol += 1;
        symbol
    }

    fn new_named_state_symbol(&mut self, name: &str) -> Result<HarnessModelSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessModelSymbol::State(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessModelSymbol::State(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    fn new_named_message_symbol(&mut self, name: &str) -> Result<HarnessModelSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessModelSymbol::Message(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessModelSymbol::Message(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    fn new_named_action_symbol(&mut self, name: &str) -> Result<HarnessModelSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessModelSymbol::Action(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessModelSymbol::Action(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    fn new_named_process_symbol(&mut self, name: &str) -> Result<HarnessModelSymbol, HarnessError> {
        let existing_symbol = self.named_symbols.get(name);
        if let Some(HarnessModelSymbol::Process(_)) = existing_symbol {
            Ok(*existing_symbol.unwrap())
        } else if let Some(_) = existing_symbol {
            Err(HarnessError::new("Another identically named symbol with mismatching type already exists"))
        } else {
            let symbol = HarnessModelSymbol::Process(self.new_symbol_id());
            self.named_symbols.insert(name.into(), symbol);
            Ok(symbol)
        }
    }

    pub fn build(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet) -> Result<HarnessModelBuild, HarnessError> {
        let mut build = HarnessModelBuild {
            states: HashMap::new(),
            messages: HashMap::new(),
            actions: HashMap::new(),
            processes: HashMap::new(),
            product_nodes: HashMap::new(),
            pending_product_mappings: Vec::new()
        };

        self.build_processes(context, process_set, &mut build)?;
        self.build_envelopes(context, &mut build)?;
        self.build_product_mappings(process_set, &mut build)?;

        Ok(build)
    }

    fn build_state(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet, build: &mut HarnessModelBuild, process: HarnessModelSymbol, symbol: HarnessModelSymbol) -> Result<StateMachineNodeID, HarnessError> {
        let node = if let Some(&node) = build.states.get(&symbol) {
            return Ok(node);
        } else {
            let state_builder = self.states.get(&symbol).expect("Expected state to exist");
            match state_builder {
                HarnessModelStateBuilder::Primitive { mnemonic } => {
                    let node = context.new_node(mnemonic)?;
                    build.states.insert(symbol, node);
                    node
                }
                
                HarnessModelStateBuilder::Product { base_state, mapped_processes } => {
                    let base_node = self.build_state(context, process_set, build, process, *base_state)?;
                    let product_builder = StateMachineProductNodeBuilder::new(base_node, mapped_processes.len());
                    let product_node = product_builder.build(context)?;
                    let node = product_node.get_root_node();
                    build.product_nodes.insert(symbol, product_node);
                    build.pending_product_mappings.push((process, symbol, mapped_processes.clone()));
                    build.states.insert(symbol, node);

                    if let Some(subnodes) = self.product_subnodes.get(&symbol) {
                        for &subnode_symbol in subnodes {
                            self.build_state(context, process_set, build, process, subnode_symbol)?;
                        }
                    }
                    node
                }

                HarnessModelStateBuilder::ProductElement { product_state, element_structure } => {
                    let product_node = build.product_nodes.get(product_state).ok_or(HarnessError::new("Unable to find product node"))?;
                    let structure = element_structure.iter()
                        .map(| symbol | build.states.get(symbol).map(| x | *x).ok_or(HarnessError::new("Unable to find state machine node")))
                        .collect::<Result<Vec<_>, _>>()?;
                    let node = product_node.get_product_subnode_for(structure)
                        .ok_or(HarnessError::new("Unable to find product node subnode"))?;
                    build.states.insert(symbol, node);
                    node
                }
            }
        };

        if let Some(edges) = self.direct_edges.get(&symbol) {
            for edge in edges {
                let edge = self.edges.get(&edge).expect("Expected edge to exist");
                let target_node = self.build_state(context, process_set, build, process, edge.target)?;
                let trigger = edge.trigger.map(| trigger | self.build_message(context, build, trigger))
                    .map_or(Ok(None), | v | v.map(Some))?;
                let action = edge.action.map(| action | self.build_action(context, build, action))
                    .map_or(Ok(None), | v | v.map(Some))?;
                context.new_edge(node, target_node, trigger, action)?;
            }
        }

        Ok(node)
    }

    fn build_message(&self, context: &mut StateMachineContext, build: &mut HarnessModelBuild, symbol: HarnessModelSymbol) -> Result<StateMachineMessageID, HarnessError> {
        if let Some(&message) = build.messages.get(&symbol) {
            Ok(message)
        } else {
            let message_builder = self.messages.get(&symbol).expect("Expected message to exist");
            let message = context.new_message(message_builder.mnemonic.clone())?;
            build.messages.insert(symbol, message);
            Ok(message)
        }
    }

    fn build_action(&self, context: &mut StateMachineContext, build: &mut HarnessModelBuild, symbol: HarnessModelSymbol) -> Result<StateMachineActionID, HarnessError> {
        if let Some(&action) = build.actions.get(&symbol) {
            Ok(action)
        } else {
            let action_builder = self.actions.get(&symbol).expect("Expected action to exist");
            let action = context.new_action(action_builder.mnemonic.clone())?;
            build.actions.insert(symbol, action);

            Ok(action)
        }
    }

    fn build_processes(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet, build: &mut HarnessModelBuild) -> Result<(), HarnessError> {
        for (&symbol, process_builder) in &self.processes {
            let entry_node = self.build_state(context, process_set, build, symbol, process_builder.entry_state)?;
            let process = process_set.new_process(process_builder.mnemonic.clone(), entry_node);
            build.processes.insert(symbol, process);
        }

        Ok(())        
    }

    fn build_product_mappings(&self, process_set: &mut ProcessSet, build: &mut HarnessModelBuild) -> Result<(), HarnessError> {
        for (process, product_node, mapped_processes) in &build.pending_product_mappings {
            let process = *build.processes.get(&process).expect("Expected process to exist");
            let processes = mapped_processes.iter()
                .map(| process | *build.processes.get(process).expect("Expected process to exist"))
                .map(| process | Into::<StateMachineMessageParticipantID>::into(process))
                .collect::<Vec<StateMachineMessageParticipantID>>();
            let product_node = build.product_nodes.get(product_node).ok_or(HarnessError::new("Unable to find product node"))?;
            let inbound_mapping = product_node.get_inbound_message_mapping(processes.clone().into_iter())?;
            let outbound_mapping = product_node.get_outbound_message_mapping(processes.into_iter())?;
            process_set.new_inbound_message_mapping(process, inbound_mapping)?;
            process_set.new_outbound_message_mapping(process, outbound_mapping)?;
        }
        build.pending_product_mappings.clear();
        Ok(())
    }

    fn build_envelopes(&self, context: &mut StateMachineContext, build: &mut HarnessModelBuild) -> Result<(), HarnessError> {
        let actions = build.actions.iter()
            .map(| (symbol, action) | (*action, self.actions.get(&symbol).expect("Expected action to exist")))
            .collect::<Vec<_>>();

        for (action, action_builder) in actions {
            for envelope in &action_builder.envelopes {
                match envelope {
                    HarnessEnvelopeBuilder::Unicast(process_symbol, behavior, message_symbol) => {
                        let process = *build.processes.get(process_symbol).expect("Expected process to exist");
                        let message = self.build_message(context, build, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Unicast(process.into()), *behavior, message)?;
                    }
    
                    HarnessEnvelopeBuilder::Multicast(process_symbols, behavior, message_symbol) => {
                        let processess = process_symbols.iter()
                            .map(| process_symbol | (*build.processes.get(process_symbol).expect("Expected process to exist")).into())
                            .collect();
                        let message = self.build_message(context, build, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Multicast(processess), *behavior, message)?;
                    }
    
                    HarnessEnvelopeBuilder::Response(behavior, message_symbol) => {
                        let message = self.build_message(context, build, *message_symbol)?;
                        context.add_envelope(action, StateMachineMessageDestination::Response, *behavior, message)?;
                    }
                }
            }
        }

        Ok(())
    }
}
