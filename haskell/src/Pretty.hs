module Pretty (
    pNil,
    pStr,
    pNewline,
    pIndent,
    pInterleave,
    pDisplay,
    Printer,
    convert,
    pretty,
) where

data Pretty = Nil
            | Str String
            | Append Pretty Pretty
            | Indent Pretty
            | Newline
            deriving (Show, Eq)

instance Semigroup Pretty where
    (<>) = Append

instance Monoid Pretty where
    mempty = Nil

class Printer a where
    {-# MINIMAL convert #-}
    convert :: a -> Pretty
    pretty :: a -> String
    pretty = pDisplay . convert

-- the interface
pNil :: Pretty
pNil = Nil
pStr :: String -> Pretty
pStr s = mconcat $ go "" s
    where
        go acc [] = [Str (reverse acc)]
        go acc (x:xs)
            | x == '\n' = Str (reverse acc) : Nil : go "" xs
            | otherwise = go (x:acc) xs
pNewline :: Pretty
pNewline = Newline
pIndent :: Pretty -> Pretty
pIndent = Indent
pInterleave :: Pretty -> [Pretty] -> Pretty
pInterleave _ [] = pNil
pInterleave _ [a] = a
pInterleave sep (x:xs) = x <> sep <> pInterleave sep xs
pDisplay :: Pretty -> String
pDisplay x = flatten 0 [(x, 0)]

flatten :: Int -> [(Pretty, Int)] -> String
flatten _ [] = ""
flatten col ((Nil, _) : xs) = flatten col xs
flatten col ((Str s, _) : xs) = s ++ flatten (col + length s) xs
flatten col ((Append left right, indent) : xs) = flatten col ((left, indent) : (right, indent) : xs)
flatten _ ((Newline, indent) : xs) = '\n' : replicate indent ' ' ++ flatten indent xs
flatten col ((Indent p, _) : xs) = flatten col ((p, col) : xs)

