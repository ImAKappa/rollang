from dataclasses import dataclass
from rollang.ecs.rollable import Rollable
import random

PENDING = None

@dataclass
class Roll:
    result: int = PENDING


class ECS:

    def __init__(self, repl_input: list[Rollable]):
        self.entities: list[int] = []
        self.amounts: list[int] = []
        self.sides: list[int] = []
        self.modifiers: list[int] = []
        self.rolls: list[Roll] = []
        for id, dice in enumerate(repl_input):
            self.entities.append(id)
            self.amounts.append(dice.amount)
            self.sides.append(dice.sides)
            self.modifiers.append(dice.modifier)
            self.rolls.append(Roll(PENDING))

    def throw_dice(self, id: int) -> Roll:
        if self.rolls[id].result == PENDING:
            return [random.randint(1, self.sides[id]) for _ in range(self.amounts[id])]
        return self.rolls[id].result

    def add_total_roll(self, id: int) -> int:
        return sum(self.rolls[id].result)

    def add_modifier(self, id: int) -> int:
        return self.modifiers[id]

    # Systems
    def roll(self):
        # Roll
        roll_results: list[int] = [0 for _ in range(len(self.entities))]
        for id in self.entities:
            # Roll, Sides, Amount
            self.rolls[id] = Roll(self.throw_dice(id))
            roll_results[id] += self.add_total_roll(id)
            # Modifier
            roll_results[id] += self.add_modifier(id)
        return roll_results


if __name__ == "__main__":
    d20 = Rollable(sides=20)
    d8 = Rollable(sides=8)
    repl_input = [d20+4, 2*d8+2]
    ecs = ECS(repl_input)
    print(f"{ecs.entities=}")
    print(f"{ecs.amounts=}")
    print(f"{ecs.sides=}")
    print(f"{ecs.modifiers=}")
    print(f"{ecs.rolls=}")
    print()
    print(ecs.roll())
    print(f"After rolling: {ecs.rolls=}")