import Control.Monad

main :: IO()

step :: Integer -> Integer
step x = if even x then down else up
    where down = div x 2
          up = 3*x+1
collatz :: Integer -> IO()
collatz 1 = do 
    putStr $ (show 1) ++ " " 
    return ()
collatz n = do 
    putStr $ (show n) ++ " "
    collatz (step n)

main = do
    line <- getLine
    let [a] = map read (words line)
    if a == 1 
      then putStr (show 1)
      else do 
        collatz a
