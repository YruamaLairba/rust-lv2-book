use lv2_atom::prelude::*;
use lv2_core::prelude::*;
use lv2_units::prelude::*;
use lv2_urid::prelude::*;

#[allow(unused_imports)]
use lv2_sys::{
    LV2_TIME__Position, LV2_TIME__Rate, LV2_TIME__Time, LV2_TIME__bar,
    LV2_TIME__barBeat, LV2_TIME__beat, LV2_TIME__beatUnit,
    LV2_TIME__beatsPerBar, LV2_TIME__beatsPerMinute, LV2_TIME__frame,
    LV2_TIME__framesPerSecond, LV2_TIME__position, LV2_TIME__speed,
    LV2_TIME_PREFIX, LV2_TIME_URI,
};

struct TimePosition;
unsafe impl UriBound for TimePosition {
    const URI: &'static [u8] = LV2_TIME__Position;
}

struct TimeBarBeat;
unsafe impl UriBound for TimeBarBeat {
    const URI: &'static [u8] = LV2_TIME__barBeat;
}

struct TimeBeatPerMinute;
unsafe impl UriBound for TimeBeatPerMinute {
    const URI: &'static [u8] = LV2_TIME__beatsPerMinute;
}

struct TimeSpeed;
unsafe impl UriBound for TimeSpeed {
    const URI: &'static [u8] = LV2_TIME__speed;
}

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
    unit_urids: UnitURIDCache,
    time_position_urid: URID<TimePosition>,
    time_barBeat_urid: URID<TimeBarBeat>,
    time_beatPerMinute_urid: URID<TimeBeatPerMinute>,
    time_speed_urid: URID<TimeSpeed>,
}

unsafe impl UriBound for Metro {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-metro-rs\0";
}

impl Metro {
    fn update_position(
        &mut self,
        object_reader: lv2_atom::object::ObjectReader,
    ) {
        println!("got time position");
        for (property_header, atom) in object_reader {
            if property_header.key == self.time_barBeat_urid {
                let val = atom.read(self.atom_urids.float, ()).unwrap();
                println!("got time barBeat : {}", val);
            }
            if property_header.key == self.time_beatPerMinute_urid {
                let val = atom.read(self.atom_urids.float, ()).unwrap();
                println!("got time beatPerMinute : {}", val);
            }
            if property_header.key == self.time_speed_urid {
                let val = atom.read(self.atom_urids.float, ()).unwrap();
                println!("got time speed : {}", val);
            }
        }
    }
}

impl Plugin for Metro {
    type Ports = Ports;

    type Features = Features<'static>;

    fn new(
        _plugin_info: &PluginInfo,
        features: Features<'static>,
    ) -> Option<Self> {
        let res = Self {
            atom_urids: features.map.populate_cache()?,
            unit_urids: features.map.populate_cache()?,
            time_position_urid: features.map.populate_cache()?,
            time_barBeat_urid: features.map.populate_cache()?,
            time_beatPerMinute_urid: features.map.populate_cache()?,
            time_speed_urid: features.map.populate_cache()?,
        };
        Some(res)
    }

    fn run(&mut self, ports: &mut Ports) {
        // Get the reading handle of the input sequence.
        let sequence = ports
            .control
            .read(self.atom_urids.sequence, self.unit_urids.beat)
            .unwrap();
        for (_timestamp, atom) in sequence {
            if let Some((header, object_reader)) =
                atom.read(self.atom_urids.object, ())
            {
                if header.otype == self.time_position_urid {
                    self.update_position(object_reader);
                }
            }
        }

        ports.output.iter_mut().for_each(|out| *out = 0f32);
    }
}

lv2_descriptors!(Metro);
