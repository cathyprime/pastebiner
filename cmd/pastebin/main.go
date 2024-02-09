package main

import (
	"bufio"
	"errors"
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

const (
	apiUrl             = "https://pastebin.com/api/api_post.php"
	loginUrl           = "https://pastebin.com/api/api_login.php"
	apiPastePrivate    = "0"
	apiPasteExpireDate = "N"
)

var (
	apiLogin       string
	apiDevKey      string
	apiUserKey     string
	apiPassword    string
	apiPasteName   string
	apiPasteFormat string
)

func getLoginKey() (string, error) {
	if loginUrl == "" || apiDevKey == "" || apiLogin == "" || apiPassword == "" {
		return "", errors.New("variable not initialized")
	}

	loginRes, err := http.PostForm(loginUrl, url.Values{
		"api_dev_key":       {apiDevKey},
		"api_user_name":     {apiLogin},
		"api_user_password": {apiPassword},
	})
	if err != nil {
		return "", err
	}

	login, err := io.ReadAll(loginRes.Body)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	return string(login), nil
}

func main() {
	consts.SetEnv()
	apiLogin = os.Getenv("APILOGIN")
	apiDevKey = os.Getenv("APIDEVKEY")
	apiPassword = os.Getenv("APIPASSWORD")

	var err error
	apiUserKey, err = getLoginKey()
	if err != nil {
		fmt.Println(err)
		return
	}

	if len(os.Args) != 2 {
		fmt.Println("provide one file name")
		return
	}
	filename := os.Args[1]
	filename, err = filepath.Abs(filename)
	if err != nil {
		fmt.Println(err)
		return
	}

	filename = filepath.Base(filename)
	file := strings.Split(filename, ".")

	if len(file) != 2 {
		fmt.Println("wrong filename")
		return
	}

	apiPasteName = file[0]
	apiPasteFormat = file[1]

	bitties, err := os.ReadFile(filename)
	if err != nil {
		fmt.Println(err)
		return
	}

	pastes, err := getPastes()
	if err != nil {
		fmt.Println(err)
		return
	}

	for _, v := range pastes {
		if v.Title == apiPasteName {
			fmt.Println("paste of the name " + apiPasteName + " exists already, delete it? [yes/No]")
			scanner := bufio.NewScanner(os.Stdin)
			if ok := scanner.Scan(); ok {
				text := scanner.Text()
				text = strings.ToLower(text)
				if text != "yes" && text != "y" {
					continue
				}

				res, err := http.PostForm(apiUrl, url.Values{
					"api_dev_key":   {apiDevKey},
					"api_user_key":  {apiUserKey},
					"api_paste_key": {v.Key},
					"api_option":    {"delete"},
				})
				if err != nil {
					fmt.Println("failed to delete")
					return
				} 
				bitties, err := httputil.DumpResponse(res, true)
				if err != nil {
					fmt.Println("failed to decode response")
					return
				}
				fmt.Println(string(bitties))
			}
		}
	}

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
