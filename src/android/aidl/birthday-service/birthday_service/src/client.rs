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
use binder::{BinderFeatures};
use com_example_birthdayservice::aidl::com::example::birthdayservice::BirthdayInfo::BirthdayInfo;
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayInfoProvider::{
    BnBirthdayInfoProvider, IBirthdayInfoProvider,
};
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayService::IBirthdayService;
use com_example_birthdayservice::binder;

const SERVICE_IDENTIFIER: &str = "birthdayservice";

/// Connect to the BirthdayService.
pub fn connect() -> Result<binder::Strong<dyn IBirthdayService>, binder::StatusCode>
{
    binder::get_interface(SERVICE_IDENTIFIER)
}

/// Call the birthday service.
fn main() -> Result<(), binder::Status> {
    let name = std::env::args().nth(1).unwrap_or_else(|| String::from("Bob"));
    let years = std::env::args()
        .nth(2)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(42);

    binder::ProcessState::start_thread_pool();
    let service = connect().expect("Failed to connect to BirthdayService");

    // Call the simple API:
    let msg = service.wishHappyBirthday(&name, years)?;
    println!("{msg}");

    // ANCHOR: wish_with_provider
    // Perform the same operation by sending an `InfoProvider` object as a `dyn
    // IBirthdayInfoProvider`.
    let provider = BnBirthdayInfoProvider::new_binder(
        InfoProvider {
            name,
            age: years as u8,
        },
        BinderFeatures::default(),
    );
    service.wishWithProvider(&provider)?;
    // ANCHOR_END: wish_with_provider

    // TODO: Perform the same operation but passing the provider as an `SpIBinder`.

    // TODO: Send a file descriptor containing the birthday info.

    Ok(())
}

// ANCHOR: InfoProvider
struct InfoProvider {
    name: String,
    age: u8,
}

impl binder::Interface for InfoProvider {}

impl IBirthdayInfoProvider for InfoProvider {
    fn name(&self) -> binder::Result<String> {
        Ok(self.name.clone())
    }

    fn years(&self) -> binder::Result<i32> {
        Ok(self.age as i32)
    }
    // ANCHOR_END: InfoProvider

    fn getInfo(&self) -> binder::Result<BirthdayInfo> {
        Ok(BirthdayInfo {
            name: self.name.clone(),
            years: self.age as i32,
        })
    }
}
