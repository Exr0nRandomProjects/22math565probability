from bitarray import bitarray
from nanoid import generate as nanoid

from typing import List, Iterable, Optional
from dataclasses import dataclass, field

@dataclass(eq=True, unsafe_hash=True, frozen=True)
class EventPossibilities:
    possibilites: bitarray
    id: str = field(default_factory=nanoid)

    def count(self):
        return self.possibilites.count('1')

    def complement(self):
        return EventPossibilities(~self.possibilites)


@dataclass(eq=True, unsafe_hash=True, frozen=True)
class Probability:
    type: str
    data: List['Probability']

    def complement(self):
        # TODO
        pass

    def generate_neighbors(self, others: Iterable['Probability']):
        match self.type:
            case 'unit':
                return [prob('')]
            case _: raise ValueError(f"invalid type {self.type}")


def prob(rpn: str, *probabilities: List['Probability']):
    stack = []
    for c in rpn:
        match c:
            case '1':
                stack.append(probabilities[0])
            case '2':
                stack.append(probabilities[1])
            case '!':
                x = stack.pop()
                stack.append(x.complement())
            case '|' | 'u' | 'n' | '+' | '*' | '-' | '/':
                b = stack.pop()
                a = stack.pop()
                stack.append(Probability(c, [a, b]))
            case _:
                raise ValueError(f"Unknown RPN character '{c}'")
    assert(len(stack) == 1)
    return stack.pop()

