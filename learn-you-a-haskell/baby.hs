doubleMe x = x + x

doubleUs :: Num a => a -> a -> a
doubleUs x y = x*2 + y*2

doubleSmallNumber :: (Ord a, Num a) => a -> a
doubleSmallNumber x  = if x > 100
    then x
    else x*2