from abc import ABCMeta, abstractmethod


class Visitor():
    """
    Visitor definition for the expression tree
    """
    __metaclass__ = ABCMeta

    def __call__(self, expr):
        return expr.accept(self)

    @abstractmethod
    def visitVariable(self, elem):
        pass

    @abstractmethod
    def visitLambda(self, elem):
        pass

    @abstractmethod
    def visitApply(self, elem):
        pass

    @abstractmethod
    def visitBottom(self, elem):
        pass

    @abstractmethod
    def visitTrue(self, elem):
        pass

    @abstractmethod
    def visitFalse(self, elem):
        pass

    @abstractmethod
    def visitIf(self, elem):
        pass

    @abstractmethod
    def visitNumber(self, elem):
        pass

    @abstractmethod
    def visitBuiltin(self, elem):
        pass

    @abstractmethod
    def visitSeq(self, elem):
        pass

    @abstractmethod
    def visitData(self, elem):
        pass
