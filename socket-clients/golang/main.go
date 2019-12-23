package main
import (
	"fmt"
	"log"
	"net"
	"sync"
)

func SendRpc() {
	fmt.Println("hello")
	conn, err := net.Dial("tcp", "172.32.0.3:60401")
	if err != nil {
		log.Fatalln(err)
	}
	defer conn.Close()
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		len, err := conn.Write([]byte("{\"method\":\"server.banner\",\"params\":[],\"id\":90003}\r\n"))
		//len, err := conn.Write([]byte("{\"method\":\"blockchain.headers.subscribe\",\"params\":[],\"id\":90003}\r\n"))
		if err != nil {
			log.Fatalln(err)
		}

		fmt.Printf("written: %d\n", len)

	}()

	wg.Wait()
	fmt.Println("sleep...")
	var response = make([]byte, 4*1024)
	for {
		len, err := conn.Read(response)
		if err != nil {
			log.Fatalln(err)
		}
		if len > 0 {
			fmt.Printf("Server> %s \n", response)
		}
	}
}

func main() {
	SendRpc()
}


