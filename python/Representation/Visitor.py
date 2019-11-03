from abc import ABCMeta, abstractmethod


class Visitor(metaclass=ABCMeta):
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
    def visitTRUE(self, elem):
        pass

    @abstractmethod
    def visitFALSE(self, elem):
        pass

    @abstractmethod
    def visitIf(self, elem):
        pass

    @abstractmethod
    def visitNumber(self, elem):
        pass

    @abstractmethod
    def visitAdd(self, elem):
        pass

    @abstractmethod
    def visitMult(self, elem):
        pass
