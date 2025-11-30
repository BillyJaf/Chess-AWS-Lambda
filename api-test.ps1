# Perform a health check:
$Params = @{
    Uri = 'http://localhost:3600/health_check'
    Method = 'Get'
}

Invoke-RestMethod @Params | ConvertTo-Json

# Send a FEN string (expect a 200 Success and legal moves bounceback):
$Params = @{
    Uri = 'http://localhost:3600/legal_moves'
    Method = 'Post'
    Body = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1' | ConvertTo-Json
    ContentType = 'application/json'
}

Invoke-RestMethod @Params | ConvertTo-Json

# Send an invalid FEN string (expect a 400 Bad Request):
$Params = @{
    Uri = 'http://localhost:3600/legal_moves'
    Method = 'Post'
    Body = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR p KQkq - 0 1' | ConvertTo-Json
    ContentType = 'application/json'
}

Invoke-RestMethod @Params | ConvertTo-Json

# Send a valid FEN string that is checkmate (expect a 200 Success and legal moves bounceback):

$Params = @{
    Uri = 'http://localhost:3600/legal_moves'
    Method = 'Post'
    Body = 'rnbqkbnr/1pppp2p/p7/5ppQ/4P3/3P4/PPP2PPP/RNB1KBNR b KQkq - 0 1' | ConvertTo-Json
    ContentType = 'application/json'
}

Invoke-RestMethod @Params | ConvertTo-Json
