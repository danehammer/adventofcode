package main

import (
	"crypto/md5"
	"fmt"
)
import "encoding/hex"
import "strconv"

// MineCoin mines an advent coin
func MineCoin(key string) int {
	i := 1
	for {
		fmt.Println(i)
		data := []byte(key + strconv.Itoa(i))
		m := md5.New()
		sum := m.Sum(data)
		sumStr := hex.EncodeToString(sum)
		fmt.Println(sumStr)
		for pos, r := range sumStr[:5] {
			if r != '0' {
				break
			}
			if pos == 4 {
				return i
			}
		}

		// if i%100 == 0 {
		//
		//
		// }

		i++
	}
}
