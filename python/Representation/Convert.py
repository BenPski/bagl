from Representation.Visitor import Visitor
import copy
import Core.Expr as C


class ExprToCore(Visitor):
    """
    Convert regular lambda calculus expressions to de bruijn notation

    have to keep track of associated indices for variables so modify some internal state

    just using a dictionary :: variable -> index
    update indices when going down one level in a lambda
    assume no shadowing of names
    if variable not found just leave it as an atom (means it is a free variable)

    when encountering a lambda have to initialize a new index in the environment
    """

    def __init__(self):
        self.env = {}

    def visitVariable(self, elem):
        if elem.s in self.env:
            return C.Index(self.env[elem.s])
        else:
            return C.Symbol(elem.s)

    def visitLambda(self, elem):
        s = elem.head.s
        self.env[s] = 0
        for key in self.env.keys():
            self.env[key] += 1
        return C.Lambda(self(elem.body))

    def visitApply(self, elem):
        self_copy = copy.deepcopy(self)
        return C.Apply(self_copy(elem.left), self_copy(elem.right))

    def visitBottom(self, elem):
        return C.Bottom()

    def visitFALSE(self, elem):
        return C.FALSE()

    def visitTRUE(self, elem):
        return C.TRUE()

    def visitIf(self, elem):
        return C.If()

    def visitNumber(self, elem):
        return C.Number(elem.n)

    def visitAdd(self, elem):
        return C.Add()

    def visitMult(self, elem):
        return C.Mult()
