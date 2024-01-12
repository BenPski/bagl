module Lib where

import GMStats
import Addr
import Assoc
import Heap
import Core
import Data.Traversable (mapAccumL)

data GMState = GMState {
    gmCode :: GMCode,
    gmStack :: GMStack,
    gmHeap :: GMHeap,
    gmGlobals :: GMGlobals,
    gmStats :: GMStats
} deriving (Eq)

instance Show GMState where
    show state 
        = ("Code: " ++ (show $ gmCode state) ++ "\n")
        ++ ("Stack: " ++ (show $ gmStack state) ++ "\n")
        ++ ("Heap: " ++ (show $ (\(_,_,x) -> x) $ gmHeap state) ++ "\n")
        ++ ("Globals: " ++ (show $ gmGlobals state) ++ "\n")
        ++ ("Stats: " ++ (show $ gmStats state) ++ "\n")
        

type GMCode = [Instruction]
type GMStack = [Addr]
type GMHeap = Heap Node
type GMGlobals = Assoc String Addr

data Instruction
    = Unwind
    | Pushglobal String
    | Pushint Int
    | Push Int
    | Mkap
    | Slide Int
    deriving (Eq, Show)

data Node
    = NNum Int
    | NAp Addr Addr
    | NGlobal Int GMCode
    deriving (Eq)

instance Show Node where
    show (NNum n) = show n
    show _ = "Not fully reduced"

run :: GMState -> Node
run state = let state' = last (eval state)
                [addr] = gmStack state'
                heap = gmHeap state'
            in hLookup heap addr


eval :: GMState -> [GMState]
eval state = state:restStates
    where
        restStates = if gmFinal state then [] else eval nextState
        nextState = doAdmin (step state)

doAdmin :: GMState -> GMState
doAdmin s = s{gmStats = statIncSteps (gmStats s)}

gmFinal :: GMState -> Bool
gmFinal s
    = case gmCode s of
        [] -> True
        _ -> False

step :: GMState -> GMState
step state = dispatch i (state{gmCode = is})
    where
        (i:is) = gmCode state

dispatch :: Instruction -> GMState -> GMState
dispatch (Pushglobal f) = pushglobal f
dispatch (Pushint n) = pushint n
dispatch Mkap = mkap
dispatch (Push n) = push n
dispatch (Slide n) = slide n
dispatch Unwind = unwind

pushglobal :: String -> GMState -> GMState
pushglobal f state = state{gmStack = a: gmStack state}
    where
        a = aLookup (gmGlobals state) f (error $ "Undeclared global " ++ f)

pushint :: Int -> GMState -> GMState
pushint n state = state{gmStack = a: gmStack state, gmHeap = heap'}
    where
        (heap', a) = hAlloc (gmHeap state) (NNum n)

mkap :: GMState -> GMState
mkap state = state{gmStack = a:as', gmHeap = heap'}
    where
        (heap', a) = hAlloc (gmHeap state) (NAp a1 a2)
        (a1:a2:as') = gmStack state

push :: Int -> GMState -> GMState
push n state = state{gmStack = a : as}
    where
        as = gmStack state
        a = getArg (hLookup (gmHeap state) (as !! (n+1)))

slide :: Int -> GMState -> GMState
slide n state = state{gmStack = a : drop n as}
    where
        (a:as) = gmStack state

unwind :: GMState -> GMState
unwind state = newState (hLookup heap a)
    where
        (a:as) = gmStack state
        heap = gmHeap state
        newState (NNum _) = state
        newState (NAp a1 a2) = state{gmCode = [Unwind], gmStack=(a1:a:as)}
        newState (NGlobal n c)
            | length as < n = error "Unwinding with too few arguments"
            | otherwise = state{gmCode=c}


getArg :: Node -> Addr
getArg (NAp a1 a2) = a2


compile :: CoreProgram -> GMState
compile program = GMState {gmCode=initialCode, gmStack=[], gmHeap=heap, gmGlobals=globals, gmStats=statInitial }
    where (heap, globals) = buildInitialHeap program

buildInitialHeap :: CoreProgram -> (GMHeap, GMGlobals)
buildInitialHeap program = mapAccumL allocateSC hInitial compiled
    where
        compiled = map compileSC program

type GMCompiledSC = (String, Int, GMCode)
type GMCompiler = CoreExpr -> GMEnvironment -> GMCode
type GMEnvironment = Assoc String Int

allocateSC :: GMHeap -> GMCompiledSC -> (GMHeap, (String, Addr))
allocateSC heap (name, nargs, insts) = (heap', (name, addr))
    where
        (heap', addr) = hAlloc heap (NGlobal nargs insts)

compileSC :: SuperCombinator -> GMCompiledSC
compileSC (name, env, body) = (name, length env, compileR body (zip env [0..]))

compileR :: GMCompiler
compileR e env = compileC e env ++ [Slide (length env + 1), Unwind]

compileC :: GMCompiler
compileC (EVar v) env
    | v `elem` aKeys env = [Push n]
    | otherwise = [Pushglobal v]
    where n = aLookup env v (error "Can't happen")
compileC (ENum n) env = [Pushint n]
compileC (EAp e1 e2) env = compileC e2 env ++ compileC e1 (argOffset 1 env) ++ [Mkap]

argOffset :: Int -> GMEnvironment -> GMEnvironment
argOffset n env = [(v, n+m) | (v,m) <- env]

initialCode :: GMCode
initialCode = [Pushglobal "main", Unwind]

compiledPrimitives :: [GMCompiledSC]
compiledPrimitives = []


