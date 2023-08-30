package main

import (
	"fmt"
	"time"

	"github.com/dreamscached/minequery/v2"
)

func main() {
	pinger := minequery.NewPinger(
		minequery.WithTimeout(5*time.Second),
		minequery.WithUseStrict(true),
		minequery.WithProtocolVersion16(minequery.Ping16ProtocolVersion162),
		minequery.WithProtocolVersion17(minequery.Ping17ProtocolVersion172),
	)

	res, err := pinger.QueryFull("play.hypixel.net", 25565)
	if err != nil {
		panic(err)
	}

	fmt.Println(res)
}
