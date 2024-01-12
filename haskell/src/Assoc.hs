module Assoc(
    Assoc,
    aLookup,
    aRemove,
    aKeys,
    aValues,
    aEmpty
)where

type Assoc a b = [(a, b)]

aLookup :: Eq a => Assoc a b -> a -> b -> b
aLookup [] k' def = def
aLookup ((k, v):xs) k' def
    | k == k' = v
    | otherwise = aLookup xs k' def

aRemove :: Eq a => Assoc a b -> a -> Assoc a b
aRemove [] _ = []
aRemove ((k, v):xs) k'
    | k == k' = xs
    | otherwise = (k, v) : aRemove xs k'

aKeys :: Assoc a b -> [a]
aKeys = map fst

aValues :: Assoc a b -> [b]
aValues = map snd

aEmpty :: Assoc a b
aEmpty = []
