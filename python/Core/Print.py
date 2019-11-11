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

    def visitBuiltin(self, elem):
        return elem.show()

    def visitData(self, elem):
        return elem.show()
