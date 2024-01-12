module Heap(
    Heap,
    hAlloc,
    hInitial,
    hUpdate,
    hFree,
    hLookup,
    hAddresses,
    hSize,
    hNull,
    hIsNull
    ) where

import Addr
import Assoc

type Heap a = (Int, [Addr], Assoc Addr a)

hInitial :: Heap a
hInitial = (0, [1..], [])

hAlloc :: Heap a -> a -> (Heap a, Addr)
hAlloc (size, next: free, cts) n = ((size+1, free, (next, n) : cts), next)
hAlloc _ _ = error "Failed to alloc to heap"

hUpdate :: Heap a -> Addr -> a -> Heap a
hUpdate (size, free, cts) a n = (size, free, (a, n) : aRemove cts a)

hFree :: Heap a -> Addr -> Heap a
hFree (size, free, cts) a = (size - 1, a:free, aRemove cts a)

hLookup :: Heap a -> Addr -> a
hLookup (size, free, cts) a = aLookup cts a (error $ "can't find " ++ show a ++ " in heap")

hAddresses :: Heap a -> [Addr]
hAddresses (size, free, cts) = map fst cts

hSize :: Heap a -> Int
hSize (size, _, _) = size

hNull :: Addr
hNull = 0

hIsNull :: Addr -> Bool
hIsNull = (==0)

