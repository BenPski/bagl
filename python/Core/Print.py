from Core.Visitor import Visitor


class Print(Visitor):
    def visitSymbol(self, elem):
        return elem.s

    def visitIndex(self, elem):
        return "#" + str(elem.n)

    def visitLambda(self, elem):
        return "(\\ " + self(elem.body) + ")"

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

    def visitNumber(self, elem):
        return str(elem.n)

    def visitBuiltin(self, elem):
        return elem.show()
