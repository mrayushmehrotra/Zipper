package main

import (
	"archive/zip"
	"io"
	"log"
	"os"
	"path/filepath"
	"time"
)

func zipSource(source, target string) error {
	startTime := time.Now() // Record the start time
	defer func() {
		elapsedTime := time.Since(startTime)
		log.Printf("Time taken to zip: %v\n", elapsedTime)
	}()

	f, err := os.Create(target)
	if err != nil {
		return err
	}
	defer f.Close()
	writer := zip.NewWriter(f)
	defer writer.Close()

	return filepath.Walk(source, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		header, err := zip.FileInfoHeader(info)
		if err != nil {
			return err
		}

		header.Method = zip.Deflate

		header.Name, err = filepath.Rel(filepath.Dir(source), path)
		if err != nil {
			return err
		}
		if info.IsDir() {
			header.Name += "/"
		}

		headerWriter, err := writer.CreateHeader(header)
		if err != nil {
			return err
		}

		if info.IsDir() {
			return nil
		}

		f, err := os.Open(path)
		if err != nil {
			return err
		}
		defer f.Close()
		_, err = io.Copy(headerWriter, f)
		return err
	})
}

func main() {
	source := "video.mp4"
	target := "zipped.zip"

	if err := zipSource(source, target); err != nil {
		log.Fatal(err)
	}
}
