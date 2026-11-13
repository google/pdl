#!/usr/bin/env python3
# Copyright 2026 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""Check that the C++ backend generates valid code for a list of PDL files.

For every input file the script:
  1. generates the C++ backend with `pdlc --output-format cxx`, and checks
     that the generator does not fail;
  2. generates stub implementations for the custom fields and checksums
     declared in the file;
  3. compiles the generated header with `-fsyntax-only`.

Files that are rejected by the PDL parser or analyzer are reported as skipped.

Example:
    cargo build --release --bin pdlc
    ./tests/cxx_compile_check.py --pdlc ../target/release/pdlc file1.pdl file2.pdl
"""

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent.parent / 'scripts'


def custom_field_stub(name: str, width: int, endianness: str) -> str:
    if width is None:
        # Unsized custom fields are stubbed as one octet long.
        width = 8
    octets = width // 8
    read = 'read_le' if endianness == 'little_endian' else 'read_be'
    write = 'write_le' if endianness == 'little_endian' else 'write_be'
    return f'''
class {name} {{
 public:
  static bool Parse(pdl::packet::slice& span, {name}* output) {{
    if (span.size() < {octets}) return false;
    output->value_ = span.{read}<uint64_t, {octets}>();
    return true;
  }}
  bool Serialize(std::vector<uint8_t>& output) const {{
    pdl::packet::Builder::{write}<uint64_t, {octets}>(output, value_);
    return true;
  }}
  size_t GetSize() const {{ return {octets}; }}
  bool operator==({name} const& other) const {{ return value_ == other.value_; }}
 private:
  uint64_t value_{{0}};
}};
'''


def checksum_stub(name: str, width: int) -> str:
    ty = {8: 'uint8_t', 16: 'uint16_t', 32: 'uint32_t', 64: 'uint64_t'}[width]
    return f'''
struct {name} {{
  {ty} value_{{0}};
  void Initialize() {{ value_ = 0; }}
  void AddByte(uint8_t byte) {{ value_ += byte; }}
  {ty} GetChecksum() const {{ return value_; }}
}};
'''


def stubs(ast: dict) -> str:
    endianness = ast['endianness']['value']
    code = ['#pragma once', '#include <cstdint>', '#include <vector>', '#include <packet_runtime.h>']
    for decl in ast['declarations']:
        if decl['kind'] == 'custom_field_declaration':
            code.append(custom_field_stub(decl['id'], decl.get('width'), endianness))
        elif decl['kind'] == 'checksum_declaration':
            code.append(checksum_stub(decl['id'], decl['width']))
    return '\n'.join(code) + '\n'


def run(cmd, **kwargs) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, capture_output=True, text=True, **kwargs)


def check(pdlc: Path, cxx: str, pdl_file: Path, workdir: Path) -> str:
    """Return the check status: 'ok', 'skipped', or an error message."""
    json_output = run([pdlc, '--output-format', 'json', pdl_file])
    if json_output.returncode != 0:
        return 'skipped'
    ast = json.loads(json_output.stdout)

    workdir.mkdir(parents=True, exist_ok=True)
    (workdir / 'stubs.h').write_text(stubs(ast))
    cxx_output = run(
        [pdlc, '--output-format', 'cxx', '--namespace', 'test', '--include-header', 'stubs.h', pdl_file])
    if cxx_output.returncode != 0 or not cxx_output.stdout:
        return 'generator failed:\n' + cxx_output.stderr.strip()
    (workdir / 'packets.h').write_text(cxx_output.stdout)
    (workdir / 'main.cc').write_text('#include "packets.h"\n')

    compile_output = run([
        cxx, '-std=c++17', '-fsyntax-only', '-ferror-limit=0', '-I', SCRIPTS_DIR, '-I', workdir,
        workdir / 'main.cc'
    ])
    if compile_output.returncode != 0:
        errors = [line for line in compile_output.stderr.splitlines() if 'error:' in line]
        return f'compilation failed ({len(errors)} errors):\n' + '\n'.join(errors[:10])
    return 'ok'


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--pdlc', type=Path, required=True, help='Path to the pdlc binary')
    parser.add_argument('--cxx', default='clang++', help='C++ compiler')
    parser.add_argument('--keep', type=Path, help='Directory where to keep the generated files')
    parser.add_argument('pdl_files', nargs='+', type=Path, help='PDL files to check')
    args = parser.parse_args()

    failures = 0
    with tempfile.TemporaryDirectory() as tmp:
        root = args.keep or Path(tmp)
        for n, pdl_file in enumerate(args.pdl_files):
            status = check(args.pdlc.resolve(), args.cxx, pdl_file, root / f'{n:03d}_{pdl_file.stem}')
            print(f'{status.splitlines()[0]:<30} {pdl_file}')
            if status not in ('ok', 'skipped'):
                failures += 1
                for line in status.splitlines()[1:]:
                    print(f'    {line}')
    print(f'{failures} failure(s)')
    sys.exit(1 if failures else 0)


if __name__ == '__main__':
    main()
