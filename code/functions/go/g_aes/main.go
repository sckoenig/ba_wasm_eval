// Implements an aes benchmark.
// First creates a random string of a given size and
// then encrypts and decrypts the string a given number of times.
//
// Inspired by:
// XFBench: A Cross-Cloud Benchmark Suite for Evaluating FaaS Workflow Platforms
// https://github.com/dream-lab/XFBench/blob/CCGRID2024/functions/text/encrypt/encrypt.py
// Vojdan Kjorveziroski & Sonja Filiposka 2023
// https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/aes/main.go
package main

import (
	"bench"
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"fmt"
	"github.com/xyproto/randomstring"
	"os"
	"strconv"
)

func main() {
	bench.Benchmark(func() {
		if len(os.Args) != 2 {
			_, _ = fmt.Fprintf(os.Stderr, "Usage: g_aes ITERATIONS")
			return
		}
		n, err := strconv.Atoi(os.Args[1])
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error parsing: %v\n", err)
			return
		}

		msg := randomstring.CookieFriendlyString(n)

		err = encryptAndDecrypt(msg, n)
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error aes: %v\n", err)
			return
		}
	})
}

func encryptAndDecrypt(msg string, iterations int) error {
	key := make([]byte, 16)
	_, _ = rand.Read(key)
	msgBytes := []byte(msg)

	for i := 0; i < iterations; i++ {
		nonce := make([]byte, 12)
		_, _ = rand.Read(nonce)

		encrypted, err := encrypt(key, msgBytes, nonce)
		if err != nil {
			return err
		}

		decrypted, err := decrypt(key, encrypted, nonce)
		if err != nil {
			return err
		}
		msgBytes = decrypted
	}
	fmt.Printf("decrypted == original text? %t \n", string(msgBytes) == msg)
	return nil
}

func encrypt(key []byte, msg []byte, nonce []byte) ([]byte, error) {
	blockCipher, err := aes.NewCipher(key)
	if err != nil {
		return nil, err
	}
	gcm, err := cipher.NewGCM(blockCipher)
	if err != nil {
		return nil, err
	}
	cipherText := gcm.Seal(nil, nonce, msg, nil)

	return cipherText, nil
}

func decrypt(key []byte, cipherText []byte, nonce []byte) ([]byte, error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return nil, err
	}
	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return nil, err
	}
	plaintext, err := aesgcm.Open(nil, nonce, cipherText, nil)

	return plaintext, err
}
