// Implements an image resize benchmark.
// Creates a thumbnail version of a given image and saves it.
//
// Inspired by:
// Vojdan Kjorveziroski & Sonja Filiposka 2023
// https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/imageprocessing/main.go
package main

import (
	"bench"
	"fmt"
	"github.com/anthonynsimon/bild/imgio"
	"github.com/anthonynsimon/bild/transform"
	"image"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	bench.Benchmark(func() {
		if len(os.Args) != 2 {
			_, _ = fmt.Fprintf(os.Stderr, "Usage: g_thumbnailer PATH/TO/IMAGE")
			return
		}
		file := os.Args[1]
		img, err := imgio.Open(file)

		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error while opening image: %s\n", err.Error())
			return
		}
		resized := resizeToFill(img, 150, 150)
		saveErr := imgio.Save(getOutPath(file), resized, imgio.JPEGEncoder(64))

		if saveErr != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error while saving image: %s\n", err.Error())
		}
	})
}

func getOutPath(path string) string {
	parentDir := filepath.Dir(path)
	fileName := strings.TrimSuffix(filepath.Base(path), filepath.Ext(path))
	return filepath.Join(parentDir, fmt.Sprintf("g_%d_thumb_%s.jpg", bench.Timestamp(), fileName))
}

// resizeToFill resizes the images to fit the larger of the given bounds,
// then cropped to fit the smaller bound.
func resizeToFill(img image.Image, width, height int) image.Image {
	orgSize := img.Bounds()
	orgWidth := orgSize.Dx()
	orgHeight := orgSize.Dy()

	// maintain ratio
	factor := max(
		float64(width)/float64(orgWidth),
		float64(height)/float64(orgHeight),
	)

	newWidth := int(float64(orgWidth) * (factor))
	newHeight := int(float64(orgHeight) * factor)

	resized := transform.Resize(
		img,
		newWidth,
		newHeight,
		transform.Lanczos,
	)

	// crop to fit boundaries
	xCrop := (resized.Bounds().Dx() - width) / 2
	yCrop := (resized.Bounds().Dy() - height) / 2
	return transform.Crop(resized, image.Rect(xCrop, yCrop, xCrop+width, xCrop+height))
}
