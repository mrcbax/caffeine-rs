caffeine-rs
===

Caffeine-rs uses the shift-release key to keep the screen awake without locking or starting a screensaver.

Cross platform is the goal with an emphasis on Windows which is what the original caffeine application was designed for.

[Original Caffeine By Zhorn Software](http://www.zhornsoftware.co.uk/caffeine/)

You can clone from GitHub or IPFS:

- GitHub: `git clone https://www.github.com/LogoiLab/caffeine-rs.git`

- IPFS: `git clone https://ipfs.io/ipfs/QmSqBKbDvmSaxYBWNsUHRDadBuAvoYNTWRJE1HmdN5rdXH/caffeine-rs` (note: IPFS version by inherent design is always at least one commit behind the GitHub version)

Caffeine takes no inputs, but it may read a configuration file if you place one in the same directory as caffeine itself. The configuration file is a toml file named `caffeine.toml`. An example `caffeine.toml` file:


> caffeine.toml
```
refresh_rate = 10
start_after = 30
exit_after = 800
active_for = 900
```
This modifies the default refresh rate to once every 10 seconds, only starts running after 30 seconds, exits after 800 seconds, sets the program active for a total of 900 seconds.

Crates Used
===

Universal
---
> chrono ![](https://img.shields.io/crates/v/chrono.svg)

> serde ![](https://img.shields.io/crates/v/serde.svg)

> toml ![](https://img.shields.io/crates/v/toml.svg)

Unix
---
> uinput ![](https://img.shields.io/crates/v/uinput.svg)

Windows
---
> keystroke ![](https://img.shields.io/crates/v/keystroke.svg)
