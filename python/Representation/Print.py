from Representation.Visitor import Visitor


class Print(Visitor):
    def visitVariable(self, elem):
        return elem.s

    def visitLambda(self, elem):
        return "(\\" + self(elem.head) + "." + self(elem.body) + ")"

    def visitApply(self, elem):
        return self(elem.left) + " " + self(elem.right)

    def visitBottom(self, elem):
        return "_|_"

    def visitFALSE(self, elem):
        return "False"

    def visitTRUE(self, elem):
        return "True"

    def visitIf(self, elem):
        return "if "
