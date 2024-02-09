package main

import (
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"

	"github.com/Yoolayn/pastebiner/internal/consts"
)

const (
	loginUrl           = "https://pastebin.com/api/api_login.php"
)

func main() {
	consts.SetEnv()
	apiLogin := os.Getenv("APILOGIN")
	apiDevKey := os.Getenv("APIDEVKEY")
	apiPassword := os.Getenv("APIPASSWORD")

	loginRes, err := http.PostForm(loginUrl, url.Values{
		"api_dev_key":       {apiDevKey},
		"api_user_name":     {apiLogin},
		"api_user_password": {apiPassword},
	})
	if err != nil {
		fmt.Println(err)
		return
	}

	apiUserKey := func() string {
		login, err := io.ReadAll(loginRes.Body)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		return string(login)
	}()
	fmt.Println(apiUserKey)
}
