//
// Copyright 2017-2023 Valve Corporation.
// Copyright 2024 phonon_rs contributors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// I kinda doubt we need these tests...
// But I haven't had to implement special array types yet...

// Test if array is created with the specified size
#[test]
fn array_size() {
    let array: [i32; 10] = [0; 10];
    assert_eq!(10, array.len());
}

// Test if array elements can be accessed correctly
#[test]
fn array_access() {
    let mut array: [i32; 10] = [0; 10];
    array[0] = 12;
    array[1] = 157;

    assert_eq!(12, array[0]);
    assert_eq!(157, array[1]);
}
