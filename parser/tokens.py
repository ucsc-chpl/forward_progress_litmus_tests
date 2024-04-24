from enum import Enum

class Token(Enum):
    THREAD  = 'THREAD' #THREAD X
    PC      = 'PC'    #/nX:
    IF      = 'IF'   # if 
    LPAREN  = 'LPAREN' #(
    RPAREN  = 'RPAREN' #)
    MEM     = 'MEM' #Mem[X]
    LBRACK  = 'LBRACK'
    RBRACK  = 'RBRACK'
    ASSIGN  = 'ASSIGN' #=
    EQUAL   = 'EQUAL' #==
    NEQUAL  = 'NEQUAL' # !=
    GOTO    = 'GOTO' #goto
    NUM     = 'NUM' #[0-9]+
    SEMI    = 'SEMI' #;
    EXCH    = 'EXCH' #Exch
    COMMA   = 'COMMA'
    IGNORE  = 'IGNORE'

class Lexeme:
    def __init__(self, token:Token, value:str) -> None:
        self.token = token
        self.value = value

    def __str__(self):
        return "(" + str(self.token) + "," + "\"" + self.value + "\"" + ")"    

def idy(l:Lexeme) -> Lexeme:
    return l

tokens = [
    (Token.THREAD,  "THREAD [0-9]+",    idy),
    (Token.PC,      "[0-9]+:",          idy),
    (Token.IF,      "if",               idy),
    (Token.LPAREN,  "(",                idy),
    (Token.RPAREN,  ")",                idy),
    (Token.MEM,     "Mem",              idy),
    (Token.LBRACK,  "[",                idy),
    (Token.RBRACK,  "]",                idy),
    (Token.ASSIGN,  "=",                idy),
    (Token.EQUAL,   "==",               idy),
    (Token.NEQUAL,  "!=",               idy),
    (Token.GOTO,    "goto",             idy),
    (Token.NUM,     "[0-9]+",           idy),
    (Token.SEMI,    ";",                idy),
    (Token.EXCH,    "Exch",             idy),
    (Token.COMMA,   ",",                idy),
    (Token,IGNORE,  " |\n",             idy),
]