from rolltoken import Token
from tokentype import TokType
from error import Error

class ParserError(Error):
    """Base-class for parsing errors"""


class Parser:

    def match_statement(self):
        return

    def match_expr(self, tokens: list[Token]):
        match tokens:
            case [TokType.DICE]:
                return TokType.DICE
            case _:
                raise ParserError("Invalid expressions")