// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use core::task::Poll;
use s2n_quic::provider::tls::s2n_tls::{ClientHelloHandler, Connection};
use std::time::Duration;
use tokio::time::Sleep;

pub struct MyClientHelloHandler {}

impl ClientHelloHandler for MyClientHelloHandler {
    fn poll_client_hello(&self, connection: &mut Connection) -> core::task::Poll<Result<(), ()>> {
        let sleep = tokio::time::sleep(Duration::from_secs(1));
        let context = core::task::Context::from_waker(con.waker());
        Box::pin(sleep).poll_unpin(&mut context).map(|_| Ok(()))
    }
}
