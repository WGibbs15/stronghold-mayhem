// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// A `Location` type used to specify where in the `Stronghold` a piece of data should be stored. A generic location
/// specifies a non-versioned location while a counter location specifies a versioned location. The Counter location can
/// be used to get the head of the version chain by passing in `None` as the counter index. Otherwise, counter records
/// are referenced through their associated index.  On Read, the `None` location is the latest record in the version
/// chain while on Write, the `None` location is the next record in the version chain.
///
/// **Note: For each used vault an encryption key is created and protected through the [libsodium](https://doc.libsodium.org/memory_management)
/// memory protection API. Many systems place limits on the amount of memory that may be locked by a process, which may
/// result in the system panicking if the upper bound is reached!
/// For users that write a large number of secrets into Stronghold, we strongly advise against writing each record in a
/// separate vault, but instead group them into a limited number of different vaults.**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
    Generic { vault_path: Vec<u8>, record_path: Vec<u8> },
    Counter { vault_path: Vec<u8>, counter: usize },
}

impl Location {
    /// Gets the vault_path from the Location.
    pub fn vault_path(&self) -> &[u8] {
        match self {
            Self::Generic { vault_path, .. } => vault_path,
            Self::Counter { vault_path, .. } => vault_path,
        }
    }

    /// Creates a generic location from types that implement [`Into<Vec<u8>>`].
    pub fn generic<V: Into<Vec<u8>>, R: Into<Vec<u8>>>(vault_path: V, record_path: R) -> Self {
        Self::Generic {
            vault_path: vault_path.into(),
            record_path: record_path.into(),
        }
    }

    /// Creates a counter location from a type that implements [`Into<Vec<u8>>`] and a counter type that implements
    /// [`Into<usize>`]
    pub fn counter<V: Into<Vec<u8>>, C: Into<usize>>(vault_path: V, counter: C) -> Self {
        Self::Counter {
            vault_path: vault_path.into(),
            counter: counter.into(),
        }
    }

    /// Used to generate a constant generic location.
    pub const fn const_generic(vault_path: Vec<u8>, record_path: Vec<u8>) -> Self {
        Self::Generic {
            vault_path,
            record_path,
        }
    }

    /// used to generate a constant counter location.
    pub const fn const_counter(vault_path: Vec<u8>, counter: usize) -> Self {
        Self::Counter { vault_path, counter }
    }
}

impl AsRef<Location> for Location {
    fn as_ref(&self) -> &Location {
        self
    }
}

/// Policy options for modifying an entire Stronghold.  Must be specified on creation.
///
/// note:
/// This is deprecated.
#[derive(Clone, Debug)]
pub enum StrongholdFlags {
    IsReadable(bool),
}

/// Policy options for for a specific vault.  Must be specified on creation.
#[derive(Clone, Debug)]
pub enum VaultFlags {}
