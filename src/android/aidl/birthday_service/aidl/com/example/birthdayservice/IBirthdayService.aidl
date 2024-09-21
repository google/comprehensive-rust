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

// ANCHOR: package
package com.example.birthdayservice;
// ANCHOR_END: package

import com.example.birthdayservice.IBirthdayInfoProvider;
import com.example.birthdayservice.BirthdayInfo;

// ANCHOR: IBirthdayService
/** Birthday service interface. */
interface IBirthdayService {
    /** Generate a Happy Birthday message. */
    String wishHappyBirthday(String name, int years);
    // ANCHOR_END: IBirthdayService

    // ANCHOR: with_info
    /** The same thing, but with a parcelable. */
    String wishWithInfo(in BirthdayInfo info);
    // ANCHOR_END: with_info

    // ANCHOR: with_info_provider
    /** The same thing, but using a binder object. */
    String wishWithProvider(IBirthdayInfoProvider provider);

    /** The same thing, but using `IBinder`. */
    String wishWithErasedProvider(IBinder provider);
    // ANCHOR_END: with_info_provider

    // ANCHOR: with_file
    /** The same thing, but loads info from a file. */
    String wishFromFile(in ParcelFileDescriptor infoFile);
    // ANCHOR_END: with_file
}
