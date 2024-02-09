package main

import (
	"fmt"
	"io"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"github.com/Yoolayn/pastebiner/internal/consts"
)

func main() {
	consts.SetEnv()
	apiUrl := os.Getenv("APIURL")
	loginUrl := os.Getenv("LOGINURL")
	apiLogin := os.Getenv("APILOGIN")
	apiDevKey := os.Getenv("APIDEVKEY")
	apiPassword := os.Getenv("APIPASSWORD")
	apiPastePrivate := os.Getenv("APIPASTEPRIVATE")
	apiPasteExpireDate := os.Getenv("APIPASTEEXPIREDATE")

	if len(os.Args) != 2 {
		fmt.Println("provide one file name")
		return
	}
	filename := os.Args[1]

	apiPasteFormat := strings.TrimPrefix(filepath.Ext(filename), ".")
	apiPasteName := strings.TrimSuffix(filename, apiPasteFormat)
	apiPasteName = strings.TrimSuffix(apiPasteName, ".")

	bitties, err := os.ReadFile(filename)
	if err != nil {
		fmt.Println(err)
		return
	}

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

	res, err := http.PostForm(apiUrl, url.Values{
		"api_dev_key":           {apiDevKey},
		"api_user_key":          {apiUserKey},
		"api_paste_private":     {apiPastePrivate},
		"api_option":            {"paste"},
		"api_paste_name":        {apiPasteName},
		"api_paste_expire_date": {apiPasteExpireDate},
		"api_paste_format":      {apiPasteFormat},
		"api_paste_code":        {string(bitties)},
	})
	if err != nil {
		fmt.Println(err)
		return
	}

	bitties, err = httputil.DumpResponse(res, true)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(string(bitties))
}
