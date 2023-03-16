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
