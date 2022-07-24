/*
   Copyright 2022 Clément Joly

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::path::Path;

use sixel::encoder::{Encoder, QuickFrameBuilder};

fn render_to_file() {
    let file_name = "./rust-logo-128x128.png";
    let path = Path::new(file_name);
    let encoder = Encoder::new().unwrap();
    encoder.set_output(Path::new("/tmp/a")).unwrap();
    encoder.encode_file(path).unwrap();
}

fn main() {
    let file_name = "./rust-logo-128x128.png";
    let path = Path::new(file_name);
    let encoder = Encoder::new().unwrap();
    let qf = QuickFrameBuilder::new().finalize();
    encoder.encode_file(path).unwrap();
}
