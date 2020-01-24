from Core.Visitor import Visitor


class Print(Visitor):
    def visitVariable(self, elem):
        return elem.s

    def visitLambda(self, elem):
        return "(\\" + self(elem.head) + " . " + self(elem.body) + ")"

    def visitApply(self, elem):
        return "(" + self(elem.left) + " " + self(elem.right) + ")"

    def visitBottom(self, elem):
        return "_|_"

    def visitTrue(self, elem):
        return "True"

    def visitFalse(self, elem):
        return "False"

    def visitIf(self, elem):
        return "if"

    def visitSeq(self, elem):
        return "seq"

    def visitNumber(self, elem):
        return str(elem.n)

    def visitString(self, elem):
        return repr(elem.s)

    def visitBuiltin(self, elem):
        return elem.show()

    def visitData(self, elem):
        return elem.show()

    def visitLetrec(self, elem):
        s = "let\n"
        for i in range(len(elem.vars)):
            s += "  " + self(elem.vars[i]) + " = " + self(elem.vals[i])
            s += "\n"
        s += "in " + self(elem.expr)
        return s

    