// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    actors::{
        secure_messages::{
            CheckRecord, CheckVault, ClearCache, DeleteFromStore, GarbageCollect, ListIds, Procedures, ReadFromStore,
            WriteToStore, WriteToVault,
        },
        RecordError, Registry,
    },
    enum_from_inner,
    procedures::{ProcedureError, ProcedureOutput},
    Location, RecordHint, RecordId,
};
use actix::prelude::*;
use futures::channel::mpsc;
use p2p::{
    firewall::FirewallRules, BehaviourState, ChannelSinkConfig, ConnectionLimits, EventChannel, InitKeypair,
    ReceiveRequest, StrongholdP2p, StrongholdP2pBuilder,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, io, time::Duration};

#[cfg(test)]
use crate::actors::secure_testing::ReadFromVault;

macro_rules! sh_result_mapping {
    ($enum:ident::$variant:ident => $inner:ty) => {
        impl From<$inner> for $enum {
            fn from(i: $inner) -> Self {
                $enum::$variant(i)
            }
        }
        impl TryFrom<$enum> for $inner {
            type Error = ();
            fn try_from(t: $enum) -> Result<Self, Self::Error> {
                if let $enum::$variant(v) = t {
                    Ok(v)
                } else {
                    Err(())
                }
            }
        }
    };
}

/// Actor that handles all network interaction.
///
/// On [`Network::new`] a new [`StrongholdP2p`] is created, which will spawn
/// a libp2p Swarm and continuously poll it.
pub struct Network {
    // Interface of stronghold-p2p for all network interaction.
    pub network: StrongholdP2p<ShRequest, ShResult>,
    // Actor registry from which the address of the target client and snapshot actor can be queried.
    pub registry: Addr<Registry>,
    // Channel through which inbound requests are received.
    // This channel is only inserted temporary on [`Network::new`], and is handed
    // to the stream handler in `<Self as Actor>::started`.
    pub _inbound_request_rx: Option<mpsc::Receiver<ReceiveRequest<ShRequest, ShResult>>>,
    // Cache the network config so it can be returned on `ExportConfig`.
    pub _config: NetworkConfig,
}

impl Network {
    pub async fn new(
        registry: Addr<Registry>,
        mut network_config: NetworkConfig,
        keypair: Option<InitKeypair>,
    ) -> Result<Self, io::Error> {
        let (firewall_tx, _) = mpsc::channel(0);
        let (inbound_request_tx, inbound_request_rx) = EventChannel::new(10, ChannelSinkConfig::BufferLatest);
        let mut builder = StrongholdP2pBuilder::new(firewall_tx, inbound_request_tx, None)
            .with_mdns_support(network_config.enable_mdns)
            .with_relay_support(network_config.enable_relay);
        if let Some(state) = network_config.state.take() {
            builder = builder.load_state(state);
        } else {
            builder = builder.with_firewall_default(FirewallRules::allow_all())
        }
        if let Some(timeout) = network_config.request_timeout {
            builder = builder.with_request_timeout(timeout)
        }
        if let Some(timeout) = network_config.connection_timeout {
            builder = builder.with_connection_timeout(timeout)
        }
        if let Some(ref limit) = network_config.connections_limit {
            builder = builder.with_connections_limit(limit.clone())
        }
        if let Some(keypair) = keypair {
            builder = builder.with_keys(keypair);
        }

        let network = builder.build().await?;
        let actor = Self {
            network,
            _inbound_request_rx: Some(inbound_request_rx),
            registry,
            _config: network_config,
        };
        Ok(actor)
    }
}

// Config for the new network.
/// Default behaviour:
/// - No limit for simultaneous connections.
/// - Request-timeout and Connection-timeout are 10s.
/// - [`Mdns`][`libp2p::mdns`] protocol is disabled. **Note**: Enabling mdns will broadcast our own address and id to
///   the local network.
/// - [`Relay`][`libp2p::relay`] functionality is disabled.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    request_timeout: Option<Duration>,
    connection_timeout: Option<Duration>,
    connections_limit: Option<ConnectionLimits>,
    enable_mdns: bool,
    enable_relay: bool,
    state: Option<BehaviourState<ShRequest>>,
}

impl NetworkConfig {
    /// Set a timeout for receiving a response after a request was sent.
    ///
    /// This applies for inbound and outbound requests.
    pub fn with_request_timeout(mut self, t: Duration) -> Self {
        self.request_timeout = Some(t);
        self
    }

    /// Set the limit for simultaneous connections.
    /// By default no connection limits apply.
    pub fn with_connections_limit(mut self, limit: ConnectionLimits) -> Self {
        self.connections_limit = Some(limit);
        self
    }

    /// Set the timeout for a idle connection to a remote peer.
    pub fn with_connection_timeout(mut self, t: Duration) -> Self {
        self.connection_timeout = Some(t);
        self
    }

    /// Enable / Disable [`Mdns`][`libp2p::mdns`] protocol.
    /// **Note**: Enabling mdns will broadcast our own address and id to the local network.
    pub fn with_mdns_enabled(mut self, is_enabled: bool) -> Self {
        self.enable_mdns = is_enabled;
        self
    }

    /// Enable / Disable [`Relay`][`libp2p::relay`] functionality.
    /// This also means that other peers can use us as relay/
    pub fn with_relay_enabled(mut self, is_enabled: bool) -> Self {
        self.enable_relay = is_enabled;
        self
    }

    /// Import state exported from a past network actor.
    pub fn load_state(mut self, state: BehaviourState<ShRequest>) -> Self {
        self.state = Some(state);
        self
    }
}

#[derive(Debug, Message, Clone, Serialize, Deserialize)]
#[rtype(result = "Result<(), RemoteRecordError>")]
pub struct WriteToRemoteVault {
    pub location: Location,
    pub payload: Vec<u8>,
    pub hint: RecordHint,
}

impl From<WriteToRemoteVault> for WriteToVault {
    fn from(t: WriteToRemoteVault) -> Self {
        let WriteToRemoteVault {
            location,
            payload,
            hint,
        } = t;
        WriteToVault {
            location,
            payload,
            hint,
        }
    }
}

impl From<WriteToVault> for WriteToRemoteVault {
    fn from(t: WriteToVault) -> Self {
        let WriteToVault {
            location,
            payload,
            hint,
        } = t;
        WriteToRemoteVault {
            location,
            payload,
            hint,
        }
    }
}

pub type RemoteRecordError = String;

// Wrapper for Requests to a remote Secure Client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShRequest {
    CheckVault(CheckVault),
    CheckRecord(CheckRecord),
    ListIds(ListIds),
    #[cfg(test)]
    ReadFromVault(ReadFromVault),
    WriteToRemoteVault(WriteToRemoteVault),
    ReadFromStore(ReadFromStore),
    WriteToStore(WriteToStore),
    DeleteFromStore(DeleteFromStore),
    GarbageCollect(GarbageCollect),
    ClearCache(ClearCache),
    Procedures(Procedures),
}

enum_from_inner!(ShRequest from CheckVault);
enum_from_inner!(ShRequest from ListIds);
#[cfg(test)]
enum_from_inner!(ShRequest from ReadFromVault);
enum_from_inner!(ShRequest from WriteToRemoteVault);
enum_from_inner!(ShRequest from ReadFromStore);
enum_from_inner!(ShRequest from WriteToStore);
enum_from_inner!(ShRequest from DeleteFromStore);
enum_from_inner!(ShRequest from GarbageCollect);
enum_from_inner!(ShRequest from ClearCache);
enum_from_inner!(ShRequest from Procedures);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShResult {
    Empty(()),
    Data(Option<Vec<u8>>),
    Bool(bool),
    WriteRemoteVault(Result<(), RemoteRecordError>),
    ListIds(Vec<(RecordId, RecordHint)>),
    Proc(Result<Vec<ProcedureOutput>, ProcedureError>),
}

sh_result_mapping!(ShResult::Empty => ());
sh_result_mapping!(ShResult::Bool => bool);
sh_result_mapping!(ShResult::Data => Option<Vec<u8>>);
sh_result_mapping!(ShResult::ListIds => Vec<(RecordId, RecordHint)>);
sh_result_mapping!(ShResult::Proc => Result<Vec<ProcedureOutput>, ProcedureError>);

impl From<Result<(), RecordError>> for ShResult {
    fn from(inner: Result<(), RecordError>) -> Self {
        ShResult::WriteRemoteVault(inner.map_err(|e| e.to_string()))
    }
}

impl TryFrom<ShResult> for Result<(), RemoteRecordError> {
    type Error = ();
    fn try_from(t: ShResult) -> Result<Self, Self::Error> {
        if let ShResult::WriteRemoteVault(result) = t {
            Ok(result)
        } else {
            Err(())
        }
    }
}
