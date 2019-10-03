# electrs with elements

## Run

`docker-compose up --build`

## Call electrs APIs

1. connect elements container  
`docker exec -it elements /bin/bash`

2. generate block  
`ecli generate`

3. call API  
`curl http://localhost:3002/blocks/tip/hash`