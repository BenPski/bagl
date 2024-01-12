module Core(
    CoreExpr(..),
    Alternative,
    SuperCombinator,
    CoreProgram
)where

data CoreExpr
    = EVar String
    | ENum Int
    | EAp CoreExpr CoreExpr
    | ELam [String] CoreExpr
    | EConstructor Int Int
    | ELet Bool [(String, CoreExpr)] CoreExpr
    | ECase CoreExpr [Alternative]
    deriving (Eq, Show)

type Alternative = (Int, [String], CoreExpr)
type SuperCombinator = (String, [String], CoreExpr)
type CoreProgram = [SuperCombinator]


