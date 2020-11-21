//! # Sled support for the `r2d2` connection pool.
//!
//! [![Latest Version](https://img.shields.io/crates/v/r2d2_sled.svg)](https://crates.io/crates/r2d2_sled)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//! [r2d2] connection pool for [sled] based on Jovansonlee Cesar's [r2d2-sqlite]. The primary use-case is to
//! enable a simple and smooth integration with the wider [r2d2] ecosystem, like [rocket_contrib]'s [databases]
//! feature.
//!
//!
//! [r2d2]: https://crates.io/crates/r2d2
//! [sled]: https://crates.io/crates/sled
//! [r2d2-sqlite]: https://crates.io/crates/r2d2-sqlite
//! [rocket_contrib]: https://docs.rs/rocket_contrib/*/rocket_contrib/
//! [databases]: https://docs.rs/rocket_contrib/*/rocket_contrib/databases/
//!
//! ## Example
//!
//! ```rust,no_run
//! use r2d2_sled::SledConnectionManager;
//!
//! fn main() {
//!     let manager = SledConnectionManager::file("my_database");
//!     let pool = r2d2::Pool::new(manager).unwrap();
//!     pool.get()
//!         .unwrap()
//!         .insert(b"yo!", b"v1")
//!         .unwrap();
//!
//!     assert_eq!(&db.get(b"yo!").unwrap().unwrap(), b"v1");
//! }
//! ```

use std::path::{Path, PathBuf};
use sled::Db;
pub use sled;

#[derive(Debug)]
enum Source {
    File(PathBuf),
    //Memory
}

/// A [`r2d2::ManageConnection`] implementation for [`sled`].
/// It is more or less just a thin wrapper around [`sled::Db`].
///
/// [`r2d2::ManageConnection`]: https://docs.rs/r2d2/0.8/r2d2/trait.ManageConnection.html
/// [`sled`]: https://docs.rs/sled/0.34/sled/
/// [`sled::Db`]: https://docs.rs/sled/0.34/sled/struct.Db.html
pub struct SledConnectionManager {
    source: Source,
}

impl SledConnectionManager {
    /// Creates a new `SledConnectionManager` from file.
    ///
    /// For more information see [`sled::open`]
    ///
    /// [`sled::open`]: https://docs.rs/sled/0.34/sled/fn.open.html
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        Self {
            source: Source::File(path.as_ref().to_path_buf()),
        }
    }
}

impl r2d2::ManageConnection for SledConnectionManager {
    type Connection = Db;
    type Error = sled::Error;

    fn connect(&self) -> Result<Db, sled::Error> {
        match self.source {
            Source::File(ref path) => sled::open(path)
        }
    }

    fn is_valid(&self, conn: &mut Db) -> Result<(), sled::Error> {
        // Should hopefully check that we are still connected to the db.
        conn.first().map(|_| ())
    }

    fn has_broken(&self, _: &mut Db) -> bool {
        false
    }
}
