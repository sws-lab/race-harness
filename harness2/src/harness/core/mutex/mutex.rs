use std::collections::{BTreeSet, HashMap, HashSet};

use crate::harness::core::{error::HarnessError, process::{ProcessID, ProcessSet}, process_state::{ProcessSetState, ProcessSetStateSpace}, state_machine::{StateMachineContext, StateMachineNodeID}};

use super::segment::MutualExclusionSegment;

pub struct ProcessSetMutualExclusion<'a> {
    process_active_states: HashMap<ProcessID, HashSet<StateMachineNodeID>>,
    process_set_state_index: HashMap<(ProcessID, StateMachineNodeID), HashSet<&'a ProcessSetState>>,
    mutually_exclusive_states: HashMap<(ProcessID, StateMachineNodeID), HashMap<ProcessID, HashSet<StateMachineNodeID>>>,
    mutual_exclusion_segments: HashSet<MutualExclusionSegment>
}

impl<'a> ProcessSetMutualExclusion<'a> {
    pub fn new(context: &'a StateMachineContext, process_set: &'a ProcessSet, state_space: &'a ProcessSetStateSpace) -> Result<ProcessSetMutualExclusion<'a>, HarnessError> {
        let mut mutual_exclusion = ProcessSetMutualExclusion {
            process_active_states: HashMap::new(),
            process_set_state_index: HashMap::new(),
            mutually_exclusive_states: HashMap::new(),
            mutual_exclusion_segments: HashSet::new()
        };
        mutual_exclusion.build(context, process_set, state_space)?;
        Ok(mutual_exclusion)
    }

    pub fn iter(&self) -> impl Iterator<Item = &MutualExclusionSegment> {
        self.mutual_exclusion_segments.iter()
    }

    fn build(&mut self, context: &'a StateMachineContext, process_set: &'a ProcessSet, state_space: &'a ProcessSetStateSpace) -> Result<(), HarnessError> {
        self.scan_process_set_states(process_set, state_space)?;
        self.scan_mutually_exclusive_states(process_set)?;
        self.generate_mutual_exclusion_segments(context, process_set)?;
        Ok(())
    }

    fn scan_process_set_states(&mut self, process_set: &'a ProcessSet, state_space: &'a ProcessSetStateSpace) -> Result<(), HarnessError> {
        for psstate in state_space.iter() {
            for process in process_set.iter() {
                let process_state = psstate.get_process_node(process).ok_or(HarnessError::new("Unable to find process state for mutual exclusion detection"))?;
                self.process_active_states.entry(process)
                    .or_insert(HashSet::new())
                    .insert(process_state);
                self.process_set_state_index.entry((process, process_state))
                    .or_insert(HashSet::new())
                    .insert(psstate);
            }
        }
        Ok(())
    }

    fn scan_mutually_exclusive_states(&mut self, process_set: &'a ProcessSet) -> Result<(), HarnessError> {
        for ((process, process_state), psstates) in &self.process_set_state_index {
            let other_processes = process_set.iter().filter(| other | other != process);
            for other_process in other_processes {
                let other_process_active_states = psstates.iter()
                    .map(| psstate | psstate.get_process_node(other_process).ok_or(HarnessError::new("Unable to find process state for mutual exclusion detection")))
                    .collect::<Result<HashSet<StateMachineNodeID>, HarnessError>>()?;
                let mutually_exclusive_states = self.process_active_states.get(&other_process)
                    .expect("Expected process active states to exist")
                    .difference(&other_process_active_states)
                    .map(| x | *x);
                self.mutually_exclusive_states.entry((*process, *process_state))
                    .or_insert(HashMap::new())
                    .insert(other_process, mutually_exclusive_states.collect());
            }
        }
        Ok(())
    }

    fn generate_mutual_exclusion_segments(&mut self, context: &'a StateMachineContext, process_set: &'a ProcessSet) -> Result<(), HarnessError> {
        let mut segments = HashSet::new();
        for process in process_set.iter() {
            segments.extend(self.generate_process_mutual_exclusion_segments(context, process_set, process)?);
        }
        self.mutual_exclusion_segments = self.prune_mutual_exclusion_segments(process_set, segments)?;
        Ok(())
    }

    fn prune_mutual_exclusion_segments(&self, process_set: &'a ProcessSet, segments: HashSet<MutualExclusionSegment>) -> Result<HashSet<MutualExclusionSegment>, HarnessError> {
        let mut segments = segments;
        let mut fixpoint_reached = false;
        while !fixpoint_reached {
            fixpoint_reached = true;

            if let Some(new_segments) = self.prune_embedded_mutual_exclusion_segments(&segments)? {
                segments = new_segments;
                fixpoint_reached = false;
                continue;
            }

            if let Some(new_segments) = self.prune_overlapping_mutual_exclusion_segments(process_set, &segments)? {
                segments = new_segments;
                fixpoint_reached = false;
                continue;
            }
        }
        Ok(segments)
    }

    fn prune_embedded_mutual_exclusion_segments(&self, segments: &HashSet<MutualExclusionSegment>) -> Result<Option<HashSet<MutualExclusionSegment>>, HarnessError> {
        for (segment_index, segment) in segments.iter().enumerate() {
            for (other_segment_index, other_segment) in segments.iter().enumerate() {
                if segment_index == other_segment_index {
                    continue;
                }

                if segment.includes(other_segment.iter()) {
                    let mut new_segments = segments.clone();
                    new_segments.remove(&other_segment);
                    return Ok(Some(new_segments));
                }
            }
        }

        Ok(None)
    }

    fn prune_overlapping_mutual_exclusion_segments(&self, process_set: &'a ProcessSet, segments: &HashSet<MutualExclusionSegment>) -> Result<Option<HashSet<MutualExclusionSegment>>, HarnessError> {
        let segment_has_process = | segment: &MutualExclusionSegment, process: ProcessID | segment.get_processes().any(| p | p == process);

        for process in process_set.get_processes() {
            for (segment_index, segment) in segments.iter().enumerate() {
                if !segment_has_process(segment, process) {
                    continue;
                }

                for (other_segment_index, other_segment) in segments.iter().enumerate() {
                    if segment_index == other_segment_index || !segment_has_process(other_segment, process) {
                        continue;
                    }

                    let mut diff_segment = segment.difference(other_segment.iter());
                    if diff_segment.get_processes().any(| p | p != process) {
                        continue;
                    }

                    for (p, s) in segment.iter() {
                        if p != process {
                            diff_segment = diff_segment.extend(p, s);
                        }
                    }

                    if &diff_segment == segment {
                        continue;
                    }

                    let mut new_segments = segments.clone();
                    new_segments.remove(segment);
                    new_segments.insert(diff_segment);
                    return Ok(Some(new_segments));
                }
            }
        }
        Ok(None)
    }

    fn generate_process_mutual_exclusion_segments(&self, context: &'a StateMachineContext, process_set: &'a ProcessSet, process: ProcessID) -> Result<HashSet<MutualExclusionSegment>, HarnessError> {
        let mut segments = self.process_initial_mutual_exclusion_segments(process_set, process)?;
        segments = self.process_propagate_mutual_exclusion_segments(context, process_set, process, segments)?;
        segments = self.process_split_mutual_exclusion_segments(process, segments)?;
        self.process_merge_state_exclusion_segments(process, segments)
    }

    fn process_initial_mutual_exclusion_segments(&self, process_set: &'a ProcessSet, process: ProcessID) -> Result<HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>, HarnessError> {
        let process_entry_node = process_set.get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
        let initial_entry_node_segment = MutualExclusionSegment::empty().union(
            self.mutually_exclusive_states.get(&(process, process_entry_node))
                .expect("Expected process to have mutual exclusive states defined")
                .iter()
                .flat_map(| (&other_process, other_process_states) | {
                    other_process_states.iter()
                        .map(move | other_process_state | (other_process, *other_process_state))
                })
        );
            
        let mut initial_segments = HashMap::new();
        initial_segments.insert(process_entry_node, HashSet::from([initial_entry_node_segment]));
        Ok(initial_segments)
    }

    fn process_propagate_mutual_exclusion_segments(&self, context: &'a StateMachineContext, process_set: &'a ProcessSet, process: ProcessID, initial_segments: HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>) -> Result<HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>, HarnessError> {
        let mut segments = initial_segments;
        let mut fixpoint_reached = false;
        while !fixpoint_reached {
            fixpoint_reached = self.process_propagate_mutual_exclusion_segments_step(context, process_set, process, &mut segments)?;
        }
        Ok(segments)
    }

    fn process_propagate_mutual_exclusion_segments_step(&self, context: &'a StateMachineContext, process_set: &'a ProcessSet, process: ProcessID, segments: &mut HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>) -> Result<bool, HarnessError> {
        let process_entry_node = process_set.get_process_entry_node(process).ok_or(HarnessError::new("Unable to determine process entry node"))?;
        let reachable_nodes = context.get_nodes_reachable_from(process_entry_node)?;
        for state in reachable_nodes.into_iter() {
            let state_segments = self.process_generate_mutual_exclusion_segments(context, process_set, process, state, segments)?;
            if !segments.contains_key(&state) || segments.get(&state).expect("Expected mutual exclusion segment to exist for a state").symmetric_difference(&state_segments).count() > 0 {
                segments.insert(state, state_segments);
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn process_generate_mutual_exclusion_segments(&self, context: &'a StateMachineContext, process_set: &'a ProcessSet, process: ProcessID, state: StateMachineNodeID, segments: &HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>) -> Result<HashSet<MutualExclusionSegment>, HarnessError> {
        let surrounding_segments = self.process_state_collect_surrounding_segments(context, process_set, process, state, segments)?;
        let mutually_exclusive_states = self.process_state_collect_mutually_exclusive_states(process, state)?;

        let mut state_segments = HashSet::new();
        let mut covered_mutually_exclusive_states = HashSet::new();
        for surrounding_segment in surrounding_segments.into_iter() {
            let segment = surrounding_segment.intersection(mutually_exclusive_states.iter().map(| x | *x));
            if !segment.is_empty() {
                covered_mutually_exclusive_states.extend(segment.iter());
                state_segments.insert(segment);
            }
        }

        let mutually_exclusive_states = mutually_exclusive_states.difference(&covered_mutually_exclusive_states)
            .map(| x | *x)
            .collect::<BTreeSet<(ProcessID, StateMachineNodeID)>>();
        if !mutually_exclusive_states.is_empty() {
            state_segments.insert(mutually_exclusive_states.into());
        }

        Ok(state_segments)
    }

    fn process_state_collect_surrounding_segments(&self, context: &'a StateMachineContext, process_set: &'a ProcessSet, process: ProcessID, state: StateMachineNodeID, segments: &HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>) -> Result<HashSet<MutualExclusionSegment>, HarnessError> {
        let mut surrounding_segments = HashSet::new();
        let edges = context.get_edges_from(state)
            .ok_or(HarnessError::new("Unable to retrieve edges coming from process state"))?;
        for edge in edges {
            let edge_target = context.get_edge_target(edge)
                .ok_or(HarnessError::new("Unable to retrieve edge target"))?;
            if let Some(target_segments) = segments.get(&edge_target) {
                surrounding_segments.extend(target_segments.iter().map(| segment | segment.clone()));
            }
        }

        let process_entry_node = process_set.get_process_entry_node(process)
            .ok_or(HarnessError::new("Unable to retrieve process entry node"))?;
        if state != process_entry_node {
            let reverse_edges = context.get_edges_to(state)
                .ok_or(HarnessError::new("Unable to retrieve edges coming from process state"))?;
            for edge in reverse_edges {
                let edge_target = context.get_edge_target(edge)
                    .ok_or(HarnessError::new("Unable to retrieve edge target"))?;
                if let Some(target_segments) = segments.get(&edge_target) {
                    surrounding_segments.extend(target_segments.iter().map(| segment | segment.clone()));
                }
            }
        }

        Ok(surrounding_segments)
    }

    fn process_state_collect_mutually_exclusive_states(&self, process: ProcessID, state: StateMachineNodeID) -> Result<HashSet<(ProcessID, StateMachineNodeID)>, HarnessError> {
        let mut mutually_exclusive_states = HashSet::new();
        if let Some(other_process_mut_ex) = self.mutually_exclusive_states.get(&(process, state)) {
            for (&other_process, other_process_states) in other_process_mut_ex {
                for &other_process_state in other_process_states {
                    mutually_exclusive_states.insert((other_process, other_process_state));
                }
            }
        }
        Ok(mutually_exclusive_states)
    }

    fn process_split_mutual_exclusion_segments(&self, process: ProcessID, segments: HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>) -> Result<HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>, HarnessError> {
        let mut new_segments = HashMap::new();
        for (state, state_segments) in segments {
            let mut new_state_segments = HashSet::new();
            for state_segment in state_segments {
                let other_processes = state_segment.iter()
                    .filter_map(| (other_process, _) | if other_process != process {
                        Some(other_process)
                    } else {
                        None
                    })
                    .collect::<HashSet<_>>();
                for other_process in other_processes {
                    let subsegment = state_segment.iter()
                        .filter(| (p, _) | *p == process || *p == other_process)
                        .collect::<BTreeSet<_>>();
                    new_state_segments.insert(subsegment.into());
                }
            }

            new_segments.insert(state, new_state_segments);
        }
        Ok(new_segments)
    }

    fn process_merge_state_exclusion_segments(&self, process: ProcessID, segments: HashMap<StateMachineNodeID, HashSet<MutualExclusionSegment>>) -> Result<HashSet<MutualExclusionSegment>, HarnessError> {
        let mut segment_index = HashMap::<MutualExclusionSegment, MutualExclusionSegment>::new();
        for (state, state_segments) in segments {
            for state_segment in state_segments {
                let new_state_segment = segment_index.get(&state_segment)
                    .or(Some(&state_segment))
                    .expect("Expected mutual exclusion segment to exist")
                    .extend(process, state);
                segment_index.insert(state_segment, new_state_segment);
            }
        }
        
        Ok(segment_index.into_iter().map(| (_, v)| v).collect())
    }
}
