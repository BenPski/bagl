{-# LANGUAGE OverloadedStrings #-}
{-# OPTIONS_GHC -Wno-unused-do-bind #-}
module Parse where

import Text.Megaparsec
import Text.Megaparsec.Char
import qualified Text.Megaparsec.Char.Lexer as L
import Data.Text
import Core
import Data.Functor.Identity (Identity)
import Data.Void (Void)
import Control.Monad.Combinators.Expr

type Parser = Parsec Void Text

spaceConsumer :: Parser ()
spaceConsumer = L.space space1 (L.skipLineComment "//") (L.skipBlockComment "/*" "*/")

lexeme :: Parser a -> Parser a
lexeme = L.lexeme spaceConsumer

symbol :: Text -> Parser Text
symbol = L.symbol spaceConsumer

name :: Parser String
name = (:) <$> lowerChar <*> many alphaNumChar

capName :: Parser String
capName = (:) <$> upperChar <*> many alphaNumChar

pInteger :: ParsecT Void Text Identity CoreExpr
pInteger = ENum <$> lexeme L.decimal

pVar :: Parser CoreExpr
pVar = EVar <$> name

pLam :: Parser CoreExpr
pLam = do
    char '\\'
    spaceConsumer
    args <- many (name <* spaceConsumer)
    char '.'
    spaceConsumer
    ELam args <$> pExpr

parens :: ParsecT Void Text Identity a -> ParsecT Void Text Identity a
parens = between (symbol "(") (symbol ")")

pTerm :: Parser CoreExpr
pTerm = choice
    [ parens pExpr
    , pInteger
    , pVar
    , pLam
    ]

pExpr :: Parser CoreExpr
pExpr = makeExprParser pTerm operatorTable

operatorTable :: [[Operator Parser CoreExpr]]
operatorTable =
    [ [ InfixL (EAp <$ symbol "")
      ]
    ]

scParse :: Parser SuperCombinator
scParse = do
    n <- name
    spaceConsumer
    args <- many (name <* spaceConsumer)
    char '='
    spaceConsumer
    body <- pExpr
    return (n, args, body)

programParse :: Parser CoreProgram
programParse = many (scParse <* (lexeme (char ';')))
