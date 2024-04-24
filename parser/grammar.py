import nltk

#sus
'''
prod_rules = """
ignore -> ' ' | '\\n'
thread -> thread num | 'THREAD'
num -> num num | '0' | '1' | '2' | '3' | '4' | '5' 
pc -> num ':'
statement -> 'IF' 'LPAREN' statement 'RPAREN'
statement -> val comp val
val -> val 'OP' val | mem | num | exch
comp -> 'EQUAL' | 'NEQUAL'
goto -> 'GOTO' num
exch -> 'EXCH' 'LPAREN' mem 'COMMA' val 'RPAREN'
mem -> 'MEM' 'LBRACK' num 'RBRACK'
"""
'''
prod_rules = """
ignore -> ' ' | '\\n'
num -> num num | '0' | '1' | '2' | '3' | '4' | '5' 
thread -> thread num | "THREAD "
pc -> num ':'
if -> 'if ' '(' statement ')'
statement -> val comp val
val -> val '+' val | mem | num | exch
comp -> ' == ' | ' != '
goto -> 'goto ' num
exch -> 'Exch' '(' mem ',' val ')'
mem -> 'Mem' '[' num ']'
"""
grammar = nltk.CFG.fromstring(prod_rules)

parser = nltk.ChartParser(grammar)
to_parse = ['THREAD ', '0', 'Exch', '(', 'Mem', ]
trees = list(parser.parse(to_parse))
print(trees[0])