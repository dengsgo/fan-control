use std::collections::HashSet;

use hardware::HardwareError;

use crate::{
    app_graph::{Node, Nodes, RootNodes},
    config::IsValid,
    id::Id,
    BoxedHardwareBridge,
};

#[derive(Debug, Clone)]
pub enum UpdateError {
    NodeNotFound,
    ValueIsNone,
    NodeIsInvalid,
    Hardware(HardwareError),
    NoInputData
}

pub struct Update {}

impl Default for Update {
    fn default() -> Self {
        Self::new()
    }
}

impl Update {
    pub fn new() -> Self {
        Self {}
    }

    pub fn graph(
        &mut self,
        nodes: &mut Nodes,
        hardware_bridge: &BoxedHardwareBridge,
        root_nodes: &RootNodes,
    ) -> Result<(), UpdateError> {
        let mut to_update: Vec<Id> = Vec::new();

        for node_id in root_nodes {
            let Some(node) = nodes.get(node_id) else {
                return Err(UpdateError::NodeNotFound);
            };

            let mut ids = Vec::new();
            if node.validate(nodes, &mut ids)? {
                to_update.extend(ids);
            };
        }

        let mut updated: HashSet<Id> = HashSet::new();

        to_update.reverse();

        for node_id in to_update {
            if !updated.contains(&node_id) {
                let Some(node) = nodes.get(&node_id) else {
                    return Err(UpdateError::NodeNotFound);
                };

                let value = node.update(nodes, hardware_bridge)?;

                let Some(node) = nodes.get_mut(&node_id) else {
                    return Err(UpdateError::NodeNotFound);
                };
                node.value = Some(value);

                updated.insert(node.id);
            }
        }

        Ok(())
    }

    pub fn clear_cache(&mut self) {}
}

impl Node {
    pub fn update(
        &self,
        nodes: &Nodes,
        hardware_bridge: &BoxedHardwareBridge,
    ) -> Result<i32, UpdateError> {
        let mut input_values = Vec::new();

        for id in &self.inputs {
            match nodes.get(id) {
                Some(node) => match node.value {
                    Some(value) => input_values.push(value),
                    None => return Err(UpdateError::ValueIsNone),
                },
                None => return Err(UpdateError::NodeNotFound),
            }
        }

        match &self.node_type {
            crate::app_graph::NodeType::Control(control) => {
                control.update(input_values[0], hardware_bridge)
            }
            crate::app_graph::NodeType::Fan(_) => todo!(),
            crate::app_graph::NodeType::Temp(_) => todo!(),
            crate::app_graph::NodeType::CustomTemp(_) => todo!(),
            crate::app_graph::NodeType::Graph(_) => todo!(),
            crate::app_graph::NodeType::Flat(flat) => Ok(flat.value.into()),
            crate::app_graph::NodeType::Linear(_) => todo!(),
            crate::app_graph::NodeType::Target(_) => todo!(),
        }
    }

    pub fn validate(&self, nodes: &Nodes, trace: &mut Vec<Id>) -> Result<bool, UpdateError> {

        if !self.is_valid() {
            return Ok(false);
        }
      
        trace.push(self.id);

        for id in &self.inputs {
            match nodes.get(id) {
                Some(node) => {
                    if !node.validate(nodes, trace)? {
                        return Ok(false);
                    }
                }
                None => return Err(UpdateError::NodeNotFound),
            }
        }

        Ok(true)
    }
}