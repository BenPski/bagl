#!/home/ben/anaconda3/bin/python
import sys

from Core.Eval import whnf
from Toaster.Parse import read
from Toaster.Convert import convert

if __name__ == "__main__":
    if len(sys.argv) == 2:
        with open(sys.argv[-1], 'r') as f:
            s = f.read()
            ast = convert(read(s))
            print(whnf(ast))
    else:
        print("usage: python bagl FILE")
