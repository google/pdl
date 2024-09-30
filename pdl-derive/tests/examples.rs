// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! PDL tests.

use pdl_derive::pdl;

#[test]
fn test_pcap() {
    #[pdl("../examples/pcap.pdl")]
    mod pcap {}

    use pcap::*;
    use pdl_runtime::Packet;

    let pcap_file = PcapFile {
        header: PcapHeader {
            version_major: 1,
            version_minor: 0,
            thiszone: 0,
            sigfigs: 0,
            snaplen: 512,
            network: 42,
        },
        records: vec![PcapRecord {
            ts_sec: 0xdead,
            ts_usec: 0xbeef,
            orig_len: 1024,
            payload: vec![1, 2, 3],
        }],
    };

    let vec = pcap_file.encode_to_vec().unwrap();
    assert!(PcapFile::decode_full(&vec).is_ok());
}

#[test]
fn test_jpeg() {
    // The JPEG syntax depends on struct inheritance which is currently
    // not supported. https://github.com/google/pdl/issues/62
    #[pdl("../examples/jpeg.pdl")]
    mod jpeg {}
}
