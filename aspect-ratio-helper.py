#!/usr/bin/env python3

# Copyright 2023 Google LLC
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
"""mdbook preprocessor which will help show the presentation aspect ratio.

The preprocessor adds a large red rectangle on every page. The
rectangle has an aspect ratio of 16 to 8.5, meaning that it is
slightly smaller than a standard 16/9 monitor.

The idea is that this approximates what the course participants can
see at once during a class. This in turn will help you estimate when
the slides are too large to be seen without scrolling.

Enable it in book.toml.
"""

import json
import sys
import textwrap


def update_book_items(items):
    aspect_ratio_helper = textwrap.dedent("""
        <style>
          div#aspect-ratio-helper {
            position: fixed;
            top: 8px;
            left: 8px;
            right: 8px;
            z-index: 1000;
            pointer-events: none;

          }

          div#aspect-ratio-helper div {
            outline: 3px dashed red;
            margin: 0 auto;
            /* At this width, the theme fonts are readable in a 16
               person conference room. If the browser is wider, the
               text becomes too small to be legible. */
            max-width: 980px;
            /* On a standard 16/9 monitor, we expect to lose a bit
               of vertical space to borders. */
            aspect-ratio: 16/8.5;
          }
        </style>

        <div id="aspect-ratio-helper">
          <div></div>
        </div>

    """)

    for item in items:
        if type(item) != dict:
            continue

        chapter = item.get('Chapter')
        if not chapter:
            continue

        chapter['content'] = aspect_ratio_helper + chapter['content']
        update_book_items(chapter['sub_items'])


if __name__ == '__main__':
    if len(sys.argv) > 1:
        if sys.argv[1] == "supports":
            sys.exit(0)

    context, book = json.load(sys.stdin)
    update_book_items(book['sections'])
    print(json.dumps(book))
