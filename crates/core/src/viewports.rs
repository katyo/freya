use dioxus_native_core::node::NodeType;
use dioxus_native_core::real_dom::NodeImmutable;

use freya_dom::prelude::FreyaDOM;
use freya_layout::Layers;

use freya_node_state::{OverflowMode, Style};
use rustc_hash::FxHashMap;

use crate::ViewportsCollection;

// Calculate all the applicable viewports for the given nodes
pub fn calculate_viewports(
    layers_nums: &[&i16],
    layers: &Layers,
    fdom: &FreyaDOM,
) -> ViewportsCollection {
    let mut viewports_collection = FxHashMap::default();
    let layout = fdom.layout();
    let rdom = fdom.rdom();

    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for node_id in layer {
            let node = rdom.get(*node_id);
            let node_areas = layout.get(*node_id);

            if let Some((node, node_areas)) = node.zip(node_areas) {
                let node_type = &*node.node_type();

                if let NodeType::Element(..) = node_type {
                    let style = node.get::<Style>().unwrap();

                    // Clip any overflow from it's children
                    if style.overflow == OverflowMode::Clip {
                        viewports_collection
                            .entry(*node_id)
                            .or_insert_with(|| (None, Vec::new()))
                            .0 = Some(node_areas.area);
                    }

                    // Pass viewports to the children
                    if let Some((_, mut inherited_viewports)) =
                        viewports_collection.get(node_id).cloned()
                    {
                        // Add the itself
                        inherited_viewports.push(*node_id);

                        for child in node.children() {
                            viewports_collection
                                .insert(child.id(), (None, inherited_viewports.clone()));
                        }
                    }
                }
            }
        }
    }
    viewports_collection
}
