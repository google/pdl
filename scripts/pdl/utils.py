# Copyright 2023 Google LLC
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

from typing import List, Tuple, Union, Optional


def indent(code: Union[str, List[str]], depth: int) -> str:
    """Indent a code block to the selected depth.

    Accepts as parameter a list of lines or a code block. Handles
    line breaks in the lines as well.
    The first line is intentionally not indented so that
    the caller may use it as:

    '''
    def generated():
        {codeblock}
    '''
    """
    code = [code] if isinstance(code, str) else code
    lines = [line for block in code for line in block.split('\n')]
    sep = '\n' + (' ' * (depth * 4))
    return sep.join(lines)


def to_pascal_case(text: str) -> str:
    """Convert UPPER_SNAKE_CASE strings to PascalCase."""
    return text.replace('_', ' ').title().replace(' ', '')
