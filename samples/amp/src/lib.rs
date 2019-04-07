// Include the lv2rs crate and the `CStr`
extern crate lv2rs;

use lv2rs::core::{self, *};
use std::ffi::CStr;

/// Every plugin defines a private structure for the plugin instance.  All data
/// associated with a plugin instance is stored here, and is available to
/// every method.  In this simple plugin, only ports need to be stored, since there is no additional
/// instance data.
struct ExAmp {
    gain: ports::ParameterInputPort,
    input: ports::AudioInputPort,
    output: ports::AudioOutputPort,
}

/// Everything a plugin needs to implement is the the `Plugin` trait. It contains all methods
/// required to make a plugin functional.
impl Plugin for ExAmp {
    /// The `instantiate` method is called by the host to create a new plugin
    /// instance.  The host passes the plugin descriptor, sample rate, and bundle
    /// path for plugins that need to load additional resources (e.g. waveforms).
    /// The features parameter contains host-provided features defined in LV2
    /// extensions, but this simple plugin does not use any.
    ///
    /// This function is in the `instantiation` threading class, so no other
    /// methods on this instance will be called concurrently with it.
    fn instantiate(
        _descriptor: &Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        _features: Option<&FeaturesList>,
    ) -> Option<Self> {
        Some(Self {
            gain: ports::ParameterInputPort::new(),
            input: ports::AudioInputPort::new(),
            output: ports::AudioOutputPort::new(),
        })
    }

    /// The `connect_port` method is called by the host to connect a particular
    /// port to a buffer.  The plugin must store the data location, but data may not
    /// be accessed except in run().
    ///
    /// In code, ports are referred to by index and since neither nor other plugins can check if the
    /// pointers are actually valid for this type, you have to absolutely make sure that you map the
    /// right number to the right port.
    ///
    /// This method is in the `audio` threading class, and is called in the same
    /// context as run().
    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.gain.connect(data as *const f32),
            1 => self.input.connect(data as *const f32),
            2 => self.output.connect(data as *mut f32),
            _ => (),
        }
    }

    /// The `activate` method is called by the host to initialise and prepare the
    /// plugin instance for running.  The plugin must reset all internal state
    /// except for buffer locations set by `connect_port()`.  Since this plugin has
    /// no other internal state, this method does nothing. You do not even have to write it out if
    /// you don't need to, since it is already provided by the trait.
    ///
    /// This method is in the `instantiation` threading class, so no other
    /// methods on this instance will be called concurrently with it.
    fn activate(&mut self) {}

    /// The `run` method is the main process function of the plugin.  It processes
    /// a block of audio in the audio context.  Since this plugin is
    /// `lv2:hardRTCapable`, `run()` must be real-time safe, so blocking (e.g. with
    /// a mutex) or memory allocation are not allowed.
    fn run(&mut self, n_samples: u32) {
        let input = unsafe { self.input.as_slice(n_samples) }.unwrap().iter();
        let output = unsafe { self.output.as_slice(n_samples) }
            .unwrap()
            .iter_mut();
        let gain = *(unsafe { self.gain.get() }.unwrap());

        // Convert the gain in dB to a coefficient.
        let coef = if gain > -90.0 {
            10.0f32.powf(gain * 0.05)
        } else {
            0.0
        };

        input
            .zip(output)
            .for_each(|(i_frame, o_frame)| *o_frame = *i_frame * coef);
    }

    /// The `deactivate` method is the counterpart to `activate`, and is called by
    /// the host after running the plugin.  It indicates that the host will not call
    /// `run` again until another call to `activate` and is mainly useful for more
    /// advanced plugins with ``live'' characteristics such as those with auxiliary
    /// processing threads.  As with `activate`, this plugin has no use for this
    /// information so this method does nothing and it is provided by the trait.
    ///
    /// This method is in the ``instantiation'' threading class, so no other
    /// methods on this instance will be called concurrently with it.
    fn deactivate(&mut self) {}

    /// The `extension_data` function returns any extension data supported by the
    /// plugin. Note that this is not an instance method, but a function on the
    /// plugin descriptor.  It is usually used by plugins to implement additional
    /// interfaces. This plugin does not have any extension data, so this function
    /// returns None. Just like `activate` and `deactivate`, this function is already provided
    /// by the trait.
    ///
    /// This method is in the ``discovery'' threading class, so no other functions
    /// or methods in this plugin library will be called concurrently with it.
    fn extension_data(_uri: &CStr) -> Option<&'static dyn ExtensionData> {
        None
    }
}

/// If you know the original LV2 book, you might ask yourself ``Where is the `cleanup`'' method?
/// Well, there is none! Instead, plugins should implement the `Drop` trait to do cleaning. When
/// the host will call for cleanup, `lv2rs` will drop the plugin.
///
/// C programs, naturally, can not work with Rust structs implementing traits. Instead, hosts look
/// for one specific function called `lv2_descriptor` which returns all required pointers.
///
/// This function is generated by this macro. It takes the name of the `lv2rs_core` sub-crate in the
/// current namespace, the identifier of the plugin struct and the URI of the plugin.
///
/// The URI is the identifier for a plugin, and how the host associates this
/// implementation in code with its description in data. If this URI does not match that used
/// in the data files, the host will fail to load the plugin.
lv2_main!(core, ExAmp, b"urn:lv2rs-book:eg-amp-rs\0");
