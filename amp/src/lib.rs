// Include the prelude of the core crate. Every specification is implemented by exactly one crate and their preludes always contain most of the types needed to use them in a plugin.
use lv2_core::prelude::*;

// Most useful plugins will have ports for input and output data. In code, these ports are represented by a struct implementing the `PortContainer` trait. Internally, ports are referred to by index. These indices are assigned in ascending order, starting with 0 for the first port. The indices in `amp.ttl` have to match them.
#[derive(PortContainer)]
struct Ports {
    gain: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

// Every plugin defines a struct for the plugin instance. All persistent data associated with a plugin instance is stored here, and is available to every instance method. In this simple plugin, there is no additional instance data and therefore, this struct is empty.
struct Amp;

// The URI is the identifier for a plugin, and how the host associates this implementation in code with its description in data. If this URI does not match that used in the data files, the host will fail to load the plugin. Since many other things are also identified by URIs, there is a separate trait for them: The `UriBound`. It stores the URI as a constant null-terminated byte slice and provides a method to easily retrieve the URI. If the null-terminator is omitted, some other parts of the system may cause undefined behaviour. Since this can not checked by Rust's type system, this trait has to be unsafe.
unsafe impl UriBound for Amp {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-amp-rs\0";
}

// Every plugin struct implements the `Plugin` trait.
impl Plugin for Amp {
    // Set the ports type.
    type Ports = Ports;

    // This plugin does not use additional host features and therefore, we set the features collection type to `()`. Other plugins define a separate struct with their required and optional features and set it here.
    type Features = ();

    // The `new` method is called by the plugin backend when it creates a new plugin instance. The host passes the plugin URI, sample rate, and bundle path for plugins that need to load additional resources (e.g. waveforms). The features parameter contains host-provided features defined in LV2 extensions, but this simple plugin does not use any. This method is in the “instantiation” threading class, so no other methods on this instance will be called concurrently with it.
    fn new(_plugin_info: &PluginInfo, _features: ()) -> Option<Self> {
        Some(Self)
    }

    // The `run()` method is the main process function of the plugin. It processes a block of audio in the audio context. Since this plugin is `lv2:hardRTCapable`, `run()` must be real-time safe, so blocking (e.g. with a mutex) or memory allocation are not allowed.
    fn run(&mut self, ports: &mut Ports) {
        let coef = if *(ports.gain) > -90.0 {
            10.0_f32.powf(*(ports.gain) * 0.05)
        } else {
            0.0
        };

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = in_frame * coef;
        }
    }
}

// The `lv2_descriptors` macro creates the entry point to the plugin library. It takes structs that implement `Plugin` and exposes them. The host will load the library and call a generated function to find all the plugins defined in the library.
lv2_descriptors!(Amp);
