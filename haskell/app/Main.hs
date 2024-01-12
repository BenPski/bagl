module Main (main) where

import Lib
import Parse
import System.Environment
import System.Exit
import Text.Megaparsec (parse, eof, errorBundlePretty)
import Data.Text as T

parseArgs ["-h"] = usage >> exit
parseArgs ["-e", expr] = return expr
parseArgs [file] = readFile file
parseArgs _ = usage >> exit

usage = putStrLn "Usage: Pass in some expressions and the main to evaluate"
exit = exitSuccess
die = exitWith $ ExitFailure 1

main :: IO ()
main = do
    args <- getArgs
    expr <- parseArgs args
    let res = run . compile <$> parse (programParse <* eof) "" (T.pack expr)
    case res of
        Right n -> print n
        Left err -> putStrLn $ errorBundlePretty err
