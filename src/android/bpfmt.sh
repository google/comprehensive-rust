#!/bin/zsh
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

# Simple wrapper for bpfmt which will remove unnecessary newlines before the
# mdbook anchors.

if ! type bpfmt > /dev/null; then
  echo 'Can not find bpfmt, do you need to run "m bpfmt"?'
  exit 1
fi

for f in comprehensive_rust/**/Android.bp; do
  bpfmt -s -w $f
  sed -zi 's|\n// ANCHOR_END|// ANCHOR_END|g' $f
done
