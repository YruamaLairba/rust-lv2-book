use lv2_atom::prelude::*;
use lv2_core::prelude::*;
use lv2_urid::prelude::*;

#[derive(PortContainer)]
pub struct Ports {
    control: InputPort<AtomPort>,
    output: OutputPort<Audio>,
}

#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: Map<'a>,
}

pub struct Metro {
    atom_urids: AtomURIDCache,
}

unsafe impl UriBound for Metro {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-metro-rs\0";
}

impl Plugin for Metro {
    type Ports = Ports;

    type Features = Features<'static>;

    fn new(_plugin_info: &PluginInfo, features: Features<'static>) -> Option<Self> {
        Some(Self {
            atom_urids: features.map.populate_cache()?,
        })
    }

    fn run(&mut self, ports: &mut Ports) {
    }
}

lv2_descriptors!(Metro);
