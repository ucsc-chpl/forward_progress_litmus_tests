import ply.lex as lex

# Todo: Add all the tokens here
tokens = [
    'THREAD',
    'PC',
    'IF',
    'ELSE',
    'GOTO', 
    'EXCH',
    'MEM',
    'NUM',
    'PLUS',
    'ASSIGN',
    'EQUAL',
    'NEQUAL',
    'LPAREN',
    'RPAREN',
    'LBRACE',
    'RBRACE',
    'LBRACK',
    'RBRACK',
    'COMMA',
    'ID', 
    'SEMI',
    'IGNORE'
]

# Todo: Add all the requied regular expression rules here
t_IF        = r'if'
t_ELSE      = r'else'
t_EXCH      = r'Exch'
t_PLUS      = r'\+'
t_ASSIGN    = r'='
t_EQUAL     = r'=='
t_NEQUAL    = r'!='
t_LPAREN    = r'\('
t_RPAREN    = r'\)'
t_LBRACE    = r'{'
t_RBRACE    = r'}'
t_LBRACK    = r'\['
t_RBRACK    = r'\]'
t_COMMA     = r'\,'
t_SEMI      = r';'
t_ignore_IGNORE    = r'\s|\n' #ignore newlines, tabs and spaces
#t_ID = r'([a-z]|[A-Z])([a-z]|[A-Z]|[0-9])*'


# Todo: Write all the required regular expression rule with action code
def t_THREAD(t):
    r'THREAD [0-9]+'
    return t

def t_PC(t):
    r'[0-9]+\:'
    t.value = int(t.value.replace(':',''))
    return t

#probably want to do something with the label!
#for now labels can only be nums!
def t_GOTO(t):
    r'goto'
    return t

def t_MEM(t):
    r'MEM'
    return t

#no decimals!
def t_NUM(t):
    r'[0-9]+'
    t.value = int(t.value)
    return t

def t_ID(t):
    r'([a-z]|[A-Z])([a-z]|[A-Z]|[0-9])+'
    if(t.value == 'if'):
        t.type = 'IF'
    elif(t.value == 'else'):
        t.type = 'ELSE'
    elif(t.value == 'Exch'):
        t.type = 'EXCH'
    elif(t.value == 'Mem'):
        t.type = 'MEM'
    elif(t.value == 'THREAD'):
        t.type = 'THREAD'
    return t

# Todo: Fix the error function 
def t_error(t):
    print(f"Illegal character: {t.value[0]}")
    t.lexer.skip(1)

# Build the lexer
lexer = lex.lex()
"""
data = '''
THREAD 0
0: if (Mem[0] == 0) goto 0;
'''

lexer.input(data)

while True:
    tok = lexer.token()
    if not tok:
        break;
    print(tok)
"""