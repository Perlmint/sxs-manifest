# SxS-manifest
Microsoft SxS assembly manifest generator

## Usage

```rust
// in build.rs

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(&{
        let mut manifest = sxs_manifest::AssemblyManifest::default();

        manifest.compatibility.max_version_tested =
            Some(sxs_manifest::manifest::windows_version::WINDOWS_10_1903);
        manifest
            .compatibility
            .supported_os
            .insert(sxs_manifest::manifest::SupportedOS::Windows10);

        manifest.serialize_to_string().unwrap()
    });
    res.compile().unwrap();
}

```