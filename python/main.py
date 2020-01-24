"""
Now need to be able to parse lets with multiple defintions
not quite sure how to do this with recursive ascent
LET
	(Var = expr)*
IN expr

want newlines to be significant in separating defintions
	line: Variable Equals Expression Newline
then this should be contained in between let and in
	Let Newline (defintions) In Expression

"""

# from Representation.Convert import convert, RightToLeftApply
from Core.Eval import whnf
from Parse.Parse import parse
from Parse.Lexer import lex
from Toaster.Parse import read
from Toaster.Convert import convert
from Core.Expr import *

if __name__ == "__main__":
	# fac = Variable("fac")
	# n = Variable("n")
	# e = Letrec([fac], [Lambda(n, Apply(Apply(Apply(If(), Apply(Apply(Equal(), n), Number(1))), Number(1)), Apply(Apply(Mult(), n), Apply(fac, Apply(Apply(Sub(), n), Number(1))))))], Apply(fac, Number(3)))
	# x = Variable("x")

	# even = Variable("even")
	# odd = Variable("odd")


	# e = Letrec([even, odd], [Lambda(x, Apply(Apply(Apply(If(), Apply(Apply(Equal(), x), Number(0))), TRUE()), Apply(odd, Apply(Apply(Sub(), x), Number(1))))),
	# 						 Lambda(x, Apply(Apply(Apply(If(), Apply(Apply(Equal(), x), Number(0))), FALSE()), Apply(even, Apply(Apply(Sub(), x), Number(1)))))], Apply(even, Number(21)))
	# main = Variable("main")
	# e = Letrec([main], [Number(3)], main)
	# print(e)
	# print(whnf(e))
    s = "let fac = (\\ n . (if (== n 1) 1 (* n (fac (- n 1))))) in fac 10"
    s = "let or = \\ x . (\\ y . (if x True y)) in (or False True)"
    # s = "let f = (\\ x . (if (!= x 0) (Cons x (f (- x 1))) Nil)) in f 10"
    with open('test.bagl', 'r') as f:
    	s = f.read()
    
    print(s)
    print(read(s))
    print(convert(read(s)))
    print(whnf(convert(read(s))))


