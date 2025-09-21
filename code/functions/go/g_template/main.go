// Implements a html template benchmark,
// creating a dynamic html output from a template and the given input.
//
// Adapted from:
// SeBS: Serverless Benchmark Suite
// https://github.com/spcl/serverless-benchmarks/tree/master/benchmarks/100.webapps/110.dynamic-html/nodejs
package main

import (
	"bench"
	"fmt"
	"github.com/flosch/pongo2/v6"
	"math/rand"
	"os"
	"strconv"
)

func main() {
	bench.Benchmark(func() {
		if len(os.Args) != 2 {
			_, _ = fmt.Fprintf(os.Stderr, "Usage: g_template COUNT")
			return
		}
		n, err := strconv.Atoi(os.Args[1])
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			return
		}

		rendered, err := renderTemplate(n)
		if err != nil {
			_, _ = fmt.Fprintf(os.Stderr, "Error rendering template:: %v\n", err)
			return
		}
		fmt.Printf("%s ... ", rendered[:min(800, len(rendered))])
	})
}

func renderTemplate(n int) (string, error) {
	templateString := `<!DOCTYPE html>
		<html>
		<head>
			<title>Randomly generated data.</title>
			<meta name="viewport" content="width=device-width, initial-scale=1.0">
			<link href="http://netdna.bootstrapcdn.com/bootstrap/3.0.0/css/bootstrap.min.css" rel="stylesheet" media="screen">
			<style type="text/css">
				.container { max-width: 500px; padding-top: 100px; }
			</style>
		</head>
		<body>
			<div class="container">
				<p>Requested: {{ n }}</p>
				<p>Requested random numbers:</p>
				<ul>
				{% for item in generated %}
					<li>{{ item }}</li>
				{% endfor %}
				</ul>
			</div>
		</body>
		</html>`

	numbers := make([]int, n)
	for i := 0; i < n; i++ {
		numbers[i] = rand.Intn(100_000)
	}
	context := pongo2.Context{
		"n":         n,
		"generated": numbers,
	}

	template, err := pongo2.FromString(templateString)
	if err != nil {
		return "", err
	}
	rendered, err := template.Execute(context)
	if err != nil {
		return "", err
	}

	return rendered, nil
}
