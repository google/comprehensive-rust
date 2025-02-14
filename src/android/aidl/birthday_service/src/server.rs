// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: main
//! Birthday service.
use birthdayservice::BirthdayService;
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayService::BnBirthdayService;
use com_example_birthdayservice::binder;
use rpcbinder::{FileDescriptorTransportMode, RpcServer};
use std::os::unix::net::UnixStream;

const SERVICE_IDENTIFIER: &str = "/tmp/birthdayservice";

/// Entry point for birthday service.
fn main() {
    let birthday_service = BirthdayService;
    let birthday_service_binder = BnBirthdayService::new_binder(
        birthday_service,
        binder::BinderFeatures::default(),
    );
    // binder::add_service(SERVICE_IDENTIFIER, birthday_service_binder.as_binder())
    //     .expect("Failed to register service");
    // binder::ProcessState::join_thread_pool();

    std::fs::write(SERVICE_IDENTIFIER, "").expect("Failed to create socket file");
    let socket = UnixStream::connect(SERVICE_IDENTIFIER).expect("Failed to create socket");
    // let socket = UnixListener::bind(SERVICE_IDENTIFIER).expect("Failed to create socket");
    let server = RpcServer::new_bound_socket(birthday_service_binder.as_binder(), socket.into()).expect("Failed to create server");
    server.set_supported_file_descriptor_transport_modes(&[FileDescriptorTransportMode::Unix]);
    server.join();
}
