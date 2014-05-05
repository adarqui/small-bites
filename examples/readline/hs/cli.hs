import System.Console.Readline

readLoop :: IO ()
readLoop = do
	maybeLine <- readline "prompt> "
	case maybeLine of
		Nothing -> return ()
		Just "q" -> return ()
		Just "ping" -> do
						putStrLn "pong"
						readLoop
		Just line -> do
						addHistory line
						putStrLn $ "got: " ++ line
						readLoop

main :: IO ()
main = do
	readLoop
