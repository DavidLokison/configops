# ConfigOps
Customizable configuration storage by traits.

Just create your configuration storage, set up your storage strategy and we take care of the rest.

```Rust
use serde_derive::{Serialize, Deserialize};
use etcetera::{choose_app_strategy, AppStrategyArgs};
use configops::{Repository, FileType};

#[derive(Default, Serialize, Deserialize)]
struct AwesomeAppConfig {
    debug: bool,
    workers: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "net".into(),
        author: "Yours Truly".into(),
        app_name: "Awesome App".into(),
    });
    let config: AwesomeAppConfig = strategy.load("config", FileType::Toml)?;
    dbg!(config);
    Ok(())
}
```

## Motivation
### Based on [`confy`](https://crates.io/crates/confy) but without the clutter
Yes, `confy` already does an excellent job at providing management for config files, but its strict dependency on a specific structure bugged me a little. You can only provide one file type feature flag, the [`etcetera`](https://crates.io/crates/etcetera) strategy is hardcoded to a specific top level domain and author and which feature flag you choose implicitely influences the file type extension.

### Trying to stay modular
So I started a new one based on `confy` which tries to overcome these obstacles. Global load and store functions to specify the whole file path if needed, a `Repository` trait that wraps around these global functions to provide a standard behaviour for loading configuration or other asset files. If the `etcetera` feature flag is enabled a blanked implementation is provided for its [`AppStrategy`](https://docs.rs/etcetera/0.11.0/etcetera/#appstrategy) trait, but you can implement your own version of the `Resolver` trait if you wish to implement custom behaviour for selecting file extensions, names or directory structure.

### More than just a config loader
This makes `configops` not only usable as a configuration file loader, but can also be used to save and retrieve file caches, store entity data, event queues or other document based files your app might need.

## License and Contribution
This work is licensed under MIT License, you acknowledge that your contributions will be published under the same terms.
