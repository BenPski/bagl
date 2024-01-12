{-# LANGUAGE NamedFieldPuns #-}
module GMStats(
    GMStats(..),
    statInitial,
    statIncSteps,
    statGetSteps
)where

newtype GMStats = GMStats {
    steps :: Int
} deriving (Eq, Show)

statInitial :: GMStats
statInitial = GMStats {steps=0}
statIncSteps :: GMStats -> GMStats
statIncSteps (GMStats {steps}) = GMStats {steps=steps + 1}
statGetSteps :: GMStats -> Int
statGetSteps = steps
