package main

import (
	"fmt"
	"time"
)

func main() {
	t, counter := time.NewTicker(time.Second), 0

	for {
		select {
		case <-t.C:
			if counter == 10 {
				fmt.Println("lost connection to pod")
				return
			}

			fmt.Println("Handled connection for 9200:200")
			counter++
		}
	}
}

// E0417 12:59:21.114449   42312 portforward.go:340] error creating error stream for port 9200 -> 9200: Timeout occurred
// E0417 12:59:21.114448   42312 portforward.go:340] error creating error stream for port 9200 -> 9200: Timeout occurred
// E0417 13:01:01.211521   42312 portforward.go:233] lost connection to pod
// "core-elasticsearch-es-default-0", "9200:9200",
//./target/debug/ppf core-elasticsearch-es-default-0 9200:9200
