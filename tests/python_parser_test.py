# Copyright 2022 Google Inc. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
"""Test if pdl.py outputs the same json as the rust pdl implementation, minus some keys"""

import unittest
import os
import stat
import subprocess
import json

KEYS_TO_REMOVE = ["loc", "comments", "file"]
FILES_TO_TEST = [
    'hci_packets.pdl',
    'packets/link_layer_packets.pdl',
]

test_dir = os.path.dirname(os.path.dirname(__file__))
pdl_bin = os.path.join(test_dir, 'pdl')
pypdl_bin = os.path.join(test_dir, 'pypdl')


def make_executable(path):
    st = os.stat(path)
    os.chmod(path, st.st_mode | stat.S_IEXEC)


make_executable(pdl_bin)
make_executable(pypdl_bin)


def simplify_ast(obj):
    """
    Remove keys that are not emitted by pypdl in ast
    """
    if isinstance(obj, dict):
        for (key, value) in list(obj.items()):
            if key in KEYS_TO_REMOVE:
                del obj[key]
            else:
                simplify_ast(value)
    elif isinstance(obj, list):
        for value in obj:
            simplify_ast(value)


class TestPdl(unittest.TestCase):

    def test_output_valid(self):
        for filename in FILES_TO_TEST:
            with self.subTest(filename):
                path = os.path.join(test_dir, filename)

                # Run pdl
                pdl = subprocess.run([pdl_bin, path], stdout=subprocess.PIPE)
                pdl_ast = json.loads(pdl.stdout)
                simplify_ast(pdl_ast)

                # Run pypdl
                pypdl = subprocess.run([pypdl_bin, path], stdout=subprocess.PIPE)
                pypdl_ast = json.loads(pypdl.stdout)

                self.maxDiff = None
                self.assertEqual(pdl_ast, pypdl_ast)


if __name__ == '__main__':
    unittest.main(verbosity=2)
