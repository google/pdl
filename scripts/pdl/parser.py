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
"""A incomplete implementation of pdl.

Made for environments where rust can not be used.
This tool only accepts valid pdl files.
Validity is checked by running the complete `pdl` tool.
"""

import re
import json
import argparse
import pathlib


def rule(reg):
    """
    Create a regex that matches multiple spaces instead of one
    """
    return reg.replace(' ', r'\s*')


def g(regex, name=None):
    """
    Create a regex group, if name is provided it will be
    a named regex group otherwise it will be a non capturing group
    """
    if name:
        return f'(?P<{name}>{regex})'
    else:
        return f'(?:{regex})'


identifier = r'\w+'
integer = r'(?:0x[a-fA-f0-9]+|[0-9]+)'
string = r'[^"]*'


def parse_fields(data):
    (data, end) = data.lstrip().split('}', 1)

    assert data[0] == '{'
    data = data[1:].strip()

    fields = []

    for field in filter(len, re.split(r'\s*,\s*', data)):
        m = re.match(g(identifier, 'name'), field)
        name = m['name']
        rest = field[m.end():]

        if name == '_checksum_start_':
            m = re.match(rule(fr' \( {g(identifier, "field_id")} \)'), rest)
            fields.append({
                'kind': 'checksum_field',
                'field_id': m['field_id'],
            })
        elif name == '_padding_':
            m = re.match(rule(fr' \[ {g(integer, "size")} \]'), rest)
            fields.append({'kind': 'padding_field', 'size': int(m['size'], 0)})
        elif name == '_size_':
            m = re.match(rule(fr' \( {g(identifier, "field_id")} \) : {g(integer, "width")}'), rest)
            fields.append({'kind': 'size_field', 'field_id': m['field_id'], 'width': int(m['width'], 0)})
        elif name == '_count_':
            m = re.match(rule(fr' \( {g(identifier, "field_id")} \) : {g(integer, "width")}'), rest)
            fields.append({'kind': 'count_field', 'field_id': m['field_id'], 'width': int(m['width'], 0)})
        elif name == '_body_':
            fields.append({
                'kind': 'body_field',
            })
        elif name == '_payload_':
            fields.append({'kind': 'payload_field', 'size_modifier': None})
        elif name == '_fixed_':
            width_or_enum = g(f'{g(integer, "width")}|{g(identifier, "enum_id")}')
            value_or_tag = g(f'{g(integer, "value")}|{g(identifier, "tag_id")}')
            m = re.match(rule(f' = {value_or_tag} : {width_or_enum}'), rest)
            fields.append({
                'kind': 'fixed_field',
                'width': int(m['width'], 0) if 'width' in m.groupdict() else None,
                'value': int(m['value'], 0) if 'value' in m.groupdict() else None,
                'enum_id': m['enum_id'],
                'tag_id': m['tag_id'],
            })
        elif name == '_reserved_':
            m = re.match(rule(f' : {g(integer, "width")}'), rest)
            fields.append({'kind': 'reserved_field', 'width': int(m['width'], 0)})
            pass
        elif rest == '':
            fields.append({
                'kind': 'group_field',
                'group_id': name,
                'constraints': [],  # TODO: parse constraints
            })
        else:
            width_or_type = g(f'{g(integer, "width")}|{g(identifier, "type_id")}')
            array = fr'\[ {g(".*", "array")} \]'
            m = re.match(rule(f' : {width_or_type} {g(array)}?'), rest)
            if m['array'] is not None:
                size, size_modifier = (int(m['array'], 0), None) if re.match(integer, m['array']) else (None,
                                                                                                        m['array'])
                fields.append({
                    'kind': 'array_field',
                    'id': name,
                    'width': int(m['width'], 0) if m['width'] else None,
                    'type_id': m['type_id'],
                    'size_modifier': size_modifier or None,
                    'size': size,
                })
            elif m['type_id']:
                fields.append({'kind': 'typedef_field', 'id': name, 'type_id': m['type_id']})
            else:
                fields.append({
                    'kind': 'scalar_field',
                    'id': name,
                    'width': int(m['width'], 0),
                })

    return fields, end


def parse_declarations(data):
    ast = {
        'version': '1,0',
        'endianness': {
            'kind': 'endianness_declaration',
            'value': 'little_endian'
        },
        'declarations': []
    }

    while data:
        [verb, data] = re.split(r'\s', data.lstrip(), 1)

        if verb == 'little_endian_packets':
            ast['endianness']['value'] = 'little_endian'
        elif verb == 'big_endian_packets':
            ast['endianness']['value'] = 'big_endian'
        elif verb == 'checksum':
            raise Exception('checksum')
        elif verb == 'custom_field':
            width = f': {g(integer, "width")}'
            m = re.match(rule(f' {g(identifier, "id")} {g(width)}? "{g(string, "function")}"'), data)
            data = data[m.end():]
            ast['declarations'].append({
                'kind': 'custom_field_declaration',
                **m.groupdict(), 'width': int(m['width'], 0)
            })
        elif verb == 'enum':
            m = re.match(rule(f' {g(identifier, "id")} : {g(integer, "width")} {{ {g("[^}]*", "tags")} }}'), data)
            data = data[m.end():]
            tags = re.split(r'\s*,\s*\n\s*', m['tags'].strip())

            def parse_tag(tag):
                m = re.match(rule(fr'(?P<id>\w+) = (?P<value>{integer})'), tag)
                return {'kind': 'tag', 'id': m['id'], 'value': int(m['value'], 0)}

            tags = list(map(parse_tag, tags))
            ast['declarations'].append({
                'kind': 'enum_declaration',
                'id': m['id'],
                'tags': tags,
                'width': int(m['width'], 0)
            })
        elif verb == 'packet' or verb == 'struct':
            parent_id = f': {g(identifier, "parent_id")}'
            constraints = fr'\( {g("[^)]*", "constraints")} \)'
            m = re.match(rule(f' {g(identifier, "id")} {g(parent_id)}? {g(constraints)}?'), data)
            data = data[m.end():]
            fields, data = parse_fields(data)

            constraints = re.split(r'\s*,\s*\n\s*', (m['constraints'] or ''))

            def parse_constraint(constraint):
                value_or_tag = g(f'{g(integer, "value")}|{g(identifier, "tag_id")}')
                m = re.match(rule(f'{g(identifier, "id")} = {value_or_tag}'), constraint)
                return {
                    'kind': 'constraint',
                    'id': m['id'],
                    'value': int(m['value'], 0) if m['value'] else None,
                    'tag_id': m['tag_id']
                }

            constraints = list(map(parse_constraint, filter(len, constraints)))

            ast['declarations'].append({
                'kind': f'{verb}_declaration',
                'id': m['id'],
                'constraints': constraints,
                'fields': fields,
                'parent_id': m['parent_id']
            })
        elif verb == 'group':
            m = re.match(rule(f' {g(identifier, "id")}'), data)
            data = data[m.end():]
            fields, data = parse_fields(data)
            ast['declarations'].append({
                'kind': f'group_declaration',
                'id': m['id'],
                'fields': fields,
            })
        elif verb == 'test':
            m = re.match(rule(f' {g(identifier, "id")} {{ {g("[^}]*", "tests")} }}'), data)
            data = data[m.end():]
        else:
            break
            raise Exception(f'unknown "{verb}"')

    return ast


def parse_file(path):
    with open(path, 'r') as file:
        data = file.read()
        data = data.strip()

        def remove_comment(line):
            return line.split(r'//', 1)[0]

        # Remove comments
        # TODO: Handle multiline comments: `/*` `*/`
        data = '\n'.join(map(remove_comment, data.split('\n')))

        return parse_declarations(data)


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument('path', help='PDL file', type=pathlib.Path)
    args = p.parse_args()
    print(json.dumps(parse_file(**vars(args))))
