# electrs

## Run bitcoin

`docker-compose up --build`

## Alternatively, run elements

`docker-compose -f docker-compose.elements.yml up --build`

## Call electrs using elements APIs

1. connect elements container  
`docker exec -it elements /bin/bash`

2. generate block where n is the number of blocks to generate
`ecli generate n`

3. call API  
`curl http://localhost:3002/blocks/tip/hash`
