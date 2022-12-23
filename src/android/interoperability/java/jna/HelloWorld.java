/*
 * Copyright 2022 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ANCHOR: HelloWorld
import com.sun.jna.Native;

public class HelloWorld {

    public static void main(String[] args) {
        RustExample example = Native.load("rust_library.dll", RustExample.class);
        var buf = ByteBuffer.allocate(100);
        example.hello(1, buf.array());
        System.out.println(Native.toString(buf.array()));
    }

    interface RustExample extends Library {
        String hello(Integer input, byte[] output_string);
    }
}