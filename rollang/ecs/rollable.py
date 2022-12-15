from dataclasses import dataclass

@dataclass
class Rollable:
    amount: int = 1
    sides: int = 1
    modifier: int = 0

    def __rmul__(self, n: int) -> None:
        """n * Rollable"""
        if isinstance(n, int):
            return Rollable(amount=self.amount * n,
                sides=self.sides,
                modifier=self.modifier,
            )
        return NotImplemented

    def __add__(self, n: int) -> None:
        """Rollable + n"""
        if isinstance(n, int):
            return Rollable(
                amount=self.amount,
                sides=self.sides,
                modifier=self.modifier+n,
            )
        return NotImplemented

if __name__ == "__main__":
    d20 = Rollable(sides=20)
    print(2*d20+4)