from dataclasses import dataclass
from typing import Tuple


@dataclass
class SizedCustomField:

    def __init__(self, value: int = 0):
        self.value = value

    def parse(span: bytes) -> Tuple['SizedCustomField', bytes]:
        return (SizedCustomField(span[0]), span[1:])

    def parse_all(span: bytes) -> 'SizedCustomField':
        assert (len(span) == 1)
        return SizedCustomField(span[0])

    @property
    def size(self) -> int:
        return 1


@dataclass
class UnsizedCustomField:

    def __init__(self, value: int = 0):
        self.value = value

    def parse(span: bytes) -> Tuple['UnsizedCustomField', bytes]:
        return (UnsizedCustomField(span[0]), span[1:])

    def parse_all(span: bytes) -> 'UnsizedCustomField':
        assert (len(span) == 1)
        return UnsizedCustomField(span[0])

    @property
    def size(self) -> int:
        return 1


def Checksum(span: bytes) -> int:
    return sum(span) % 256
