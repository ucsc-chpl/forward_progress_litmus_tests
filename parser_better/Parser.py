import argparse
import ply.yacc as yacc

# Importing tokens map from part1Lexer
from Lexer import lexer, tokens

# Todo: Define all the required grammar and actions
def p_program(p):
    'program : threads'

def p_threads(p):
    '''threads : thread threads
               | thread'''

def p_thread(p):
    '''thread : THREAD NUM statement_list'''

# don't like this but I don't know a better way??
def p_statement_list(p):
    '''statement_list : PC statement statement_list
                      | PC statement'''
#how do I define a line?
def p_statement(p):
    '''statement : simple_stmts
                 | compound_stmt'''
#sus
def p_simple_stmts(p):
    '''simple_stmts : simple_stmt SEMI'''
#thats all?
def p_simple_stmt(p):
    '''simple_stmt : assign
                   | goto_stmt'''

def p_compound_stmt(p):
    '''compound_stmt : if_stmt
                     | '''

def p_assign(p):
    '''assign : ID ASSIGN expr
              |'''

def p_goto_stmt(p):
    '''goto_stmt : GOTO NUM'''

def p_expr(p):
    '''expr : expr PLUS val
            | val'''
def p_val(p):
    '''val : mem
           | exch
           | NUM'''
def p_exch(p):
    '''exch : EXCH LPAREN mem COMMA NUM RPAREN'''
#math inside mem not supported lol
def p_mem(p):
    '''mem : MEM LBRACK NUM RBRACK'''

def p_compare(p):
    '''compare : expr EQUAL expr
              | expr NEQUAL expr'''

def p_if_stmt(p):
    '''if_stmt : IF LPAREN compare RPAREN statement
               | '''
def p_error(p):
    if p:
        print(f"Syntax error at {p.value}")
    else:
        print("Syntax error at EOF")

def test_parser(input_string):
    result = parser.parse(input_string, lexer=lexer)
    if result is None:
        print("Input matches the grammar.")
    else:
        print("Input does not match the grammar.")

# Build the parser
ply_parser = yacc.yacc(debug=True)

if __name__ == "__main__":
    arg_parser = argparse.ArgumentParser()
    arg_parser.add_argument('file_name', type=str, help="Input file containing the text to parse.")
    args = arg_parser.parse_args()
    
    try:
        with open(args.file_name, 'r') as file:
            f_contents = file.read()
            #print("File contents read successfully:")
            #print(f_contents)
    except FileNotFoundError:
        print(f"Error: File '{args.file_name}' not found.")
        exit()

    try:
        result = ply_parser.parse(f_contents, lexer=lexer)
        if result is None:
            print("Input matches the grammar.")
        else:
            print("Input does not match the grammar.")
    except Exception as e:
        print(f"Error during parsing: {e}")