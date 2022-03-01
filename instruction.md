##  generate a random number
near call arthurkamau.testnet generate --accountId arthurkamau.testnet

##  guess a  nuber between 1 and 10
near call arthurkamau.testnet guess'{"input": 5}' --accountId arthurkamau.testnet

##  if wrong get suggestion if too big or too small
##  we can get the guess  
near call arthurkamau.testnet get_num --accountId arthurkamau.testnet

##  from previous result
near call arthurkamau.testnet guess'{"input": 5}' --accountId arthurkamau.testnet

##  if right number is reset.
near call arthurkamau.testnet get_num --accountId arthurkamau.testnet
