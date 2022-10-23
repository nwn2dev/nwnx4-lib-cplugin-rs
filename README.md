
# NWNX4 CPlugin Rust bindings

Provides a safe interface for writing CPlugins in Rust

## Example

`Cargo.toml`
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
log = "0.4.17"
nwnx4-lib-cplugin-rs = { git = "https://github.com/nwn2dev/nwnx4-lib-cplugin-rs.git" }

```

`lib.rs`
```rs
// Include CPlugin stuff
use nwnx4_lib_cplugin_rs::{cplugin_hook, CPlugin, InitInfo};

// Bind NWNX4 functions to YourPluginStruct
// See `enum CPluginEndpoints` for a list of bindable functions
cplugin_hook!(YourPluginStruct, [get_id, get_info, get_version, get_string, set_string]);

// Your custom plugin data
struct YourPluginStruct {}

// Implement bound CPlugin functions
impl<'a> CPlugin<'a> for YourPluginStruct {
	// You must implement this function
	fn new(info: InitInfo) -> Result<Self, Box<dyn Error>> {
		Ok(Self {})
	}

    // Other functions have a default implementation, that you can override:
	fn get_id(&mut self) -> &'a CStr {
        &CStr::from_bytes_with_nul(b"YOURPLUGIN\0").unwrap()
    }
}
```
