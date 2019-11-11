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
    def visitLambdaM(self, elem):
        pass

    @abstractmethod
    def visitApply(self, elem):
        pass

    @abstractmethod
    def visitApplyM(self, elem):
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
    def visitSeq(self, elem):
        pass

    @abstractmethod
    def visitNumber(self, elem):
        pass

    @abstractmethod
    def visitAdd(self, elem):
        pass

    @abstractmethod
    def visitSub(self, elem):
        pass

    @abstractmethod
    def visitMult(self, elem):
        pass

    @abstractmethod
    def visitEqual(self, elem):
        pass

    @abstractmethod
    def visitLet(self, elem):
        pass

    @abstractmethod
    def visitLetRec(self, elem):
        pass

    @abstractmethod
    def visitNil(self, elem):
        pass

    @abstractmethod
    def visitCons(self, elem):
        pass

    @abstractmethod
    def visitHead(self, elem):
        pass

    @abstractmethod
    def visitTail(self, elem):
        pass
