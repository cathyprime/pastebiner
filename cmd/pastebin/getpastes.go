package main

import (
	"encoding/xml"
	"errors"
	"io"
	"net/http"
	"net/url"
	"strconv"
)

type Paste struct {
	XMLName     xml.Name `xml:"paste"`
	Key         string   `xml:"paste_key"`
	Date        int      `xml:"paste_date"`
	Title       string   `xml:"paste_title"`
	Size        int      `xml:"paste_size"`
	ExpireDate  int      `xml:"paste_expire_date"`
	Private     int      `xml:"paste_private"`
	FormatLong  string   `xml:"paste_format_long"`
	FormatShort string   `xml:"paste_format_short"`
	Url         string   `xml:"paste_url"`
	Hits        int      `xml:"paste_hits"`
}

type Root struct {
	XMLName xml.Name `xml:"root"`
	Pastes  []Paste  `xml:"paste"`
}

func getPastes() ([]Paste, error) {
	res, err := http.PostForm(apiUrl, url.Values{
		"api_option":   {"list"},
		"api_user_key": {apiUserKey},
		"api_dev_key":  {apiDevKey},
	})
	if err != nil {
		return []Paste{}, err
	}

	switch res.StatusCode {
	case http.StatusOK:
		fallthrough
	case http.StatusAccepted:
	default:
		return []Paste{}, errors.New("request failed " + strconv.Itoa(res.StatusCode))
	}

	bitties, err := io.ReadAll(res.Body)
	if err != nil {
		return []Paste{}, err
	}

	bitties = []byte("<root>" + string(bitties) + "</root>")

	var root Root
	err = xml.Unmarshal(bitties, &root)
	if err != nil {
		return []Paste{}, err
	}

	return root.Pastes, err
}
