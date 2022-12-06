import System.IO
import Data.List

main = do
  contents <- readFile "input.txt"

  let p1 = findHeader contents 0 4
  let p2 = findHeader contents 0 14

  putStrLn $ "p1: " ++ show p1
  putStrLn $ "p2: " ++ show p2

findHeader :: String -> Int -> Int -> (Int)
findHeader str index size =
  if length header == size && length (nub header) == size
    then index + size
    else findHeader (tail str) (index + 1) size
  where
    header = take size str
