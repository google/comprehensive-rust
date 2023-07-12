#!/bin/bash
# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -e

function run_example() {
  while read -r line; do
    if [[ "$line" != \#* ]]; then
      echo "$line"
      eval "${line#$ }"
    fi
  done
}

cd "$ANDROID_BUILD_TOP"
source build/envsetup.sh
lunch aosp_cf_x86_64_phone-userdebug
#acloud reconnect --autoconnect adb

adb root
adb shell rm -rf '/data/local/tmp/*'

run_example <<EOF
# ANCHOR: hello_rust
m hello_rust
adb push "$ANDROID_PRODUCT_OUT/system/bin/hello_rust /data/local/tmp"
adb shell /data/local/tmp/hello_rust
# ANCHOR_END: hello_rust
EOF

run_example <<EOF
# ANCHOR: hello_rust_with_dep
m hello_rust_with_dep
adb push "$ANDROID_PRODUCT_OUT/system/bin/hello_rust_with_dep /data/local/tmp"
adb shell /data/local/tmp/hello_rust_with_dep
# ANCHOR_END: hello_rust_with_dep
EOF

function birthday_server() {
  run_example <<EOF
# ANCHOR: birthday_server
m birthday_server
adb push "$ANDROID_PRODUCT_OUT/system/bin/birthday_server /data/local/tmp"
adb shell /data/local/tmp/birthday_server
# ANCHOR_END: birthday_server
EOF
}

pkill -f birthday_server || true
birthday_server &
BIRTHDAY_SERVER_PID=$!

while adb shell service check birthdayservice | grep -q 'not found'; do
  echo "Waiting on birthdayservice..."
  sleep 3
done
echo "Found birthdayservice..."

run_example <<EOF
# ANCHOR: service_check_birthday_server
adb shell service check birthdayservice
# ANCHOR_END: service_check_birthday_server
EOF

run_example <<EOF
# ANCHOR: service_call_birthday_server
adb shell service call birthdayservice 1 s16 Bob i32 24
# ANCHOR_END: service_call_birthday_server
EOF

run_example <<EOF
# ANCHOR: birthday_client
m birthday_client
adb push "$ANDROID_PRODUCT_OUT/system/bin/birthday_client /data/local/tmp"
adb shell /data/local/tmp/birthday_client Charlie 60
# ANCHOR_END: birthday_client
EOF

pkill -f birthday_server


run_example <<EOF
# ANCHOR: hello_rust_logs
m hello_rust_logs
adb push "$ANDROID_PRODUCT_OUT/system/bin/hello_rust_logs /data/local/tmp"
adb shell /data/local/tmp/hello_rust_logs
# ANCHOR_END: hello_rust_logs
EOF

run_example <<EOF
# ANCHOR: print_birthday_card
m print_birthday_card
adb push "$ANDROID_PRODUCT_OUT/system/bin/print_birthday_card /data/local/tmp"
adb shell /data/local/tmp/print_birthday_card
# ANCHOR_END: print_birthday_card
EOF

run_example <<EOF
# ANCHOR: libbirthday_bindgen_test
atest libbirthday_bindgen_test
# ANCHOR_END: libbirthday_bindgen_test
EOF

run_example <<EOF
# ANCHOR: analyze_numbers
m analyze_numbers
adb push "$ANDROID_PRODUCT_OUT/system/bin/analyze_numbers /data/local/tmp"
adb shell /data/local/tmp/analyze_numbers
# ANCHOR_END: analyze_numbers
EOF

run_example <<EOF
# ANCHOR: helloworld_jni
m helloworld_jni
adb sync  # requires adb root && adb remount
adb shell /system/bin/helloworld_jni
# ANCHOR_END: helloworld_jni
EOF
