# r2d2-sled

[![Latest Version](https://img.shields.io/crates/v/r2d2_sled.svg)](https://crates.io/crates/r2d2_sled)
[![Documentation](https://docs.rs/r2d2_sled/badge.svg)](https://docs.rs/r2d2_sled)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

[r2d2] connection pool for [sled] based on Jovansonlee Cesar's [r2d2-sqlite]. The primary use-case is to
enable a simple and smooth integration with the wider [r2d2] ecosystem, like [rocket_contrib]'s [databases]
feature.

[r2d2]: https://crates.io/crates/r2d2
[sled]: https://crates.io/crates/sled
[r2d2-sqlite]: https://crates.io/crates/r2d2-sqlite
[rocket_contrib]: https://docs.rs/rocket_contrib/*/rocket_contrib/
[databases]: https://docs.rs/rocket_contrib/*/rocket_contrib/databases/
