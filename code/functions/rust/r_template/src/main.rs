//! Implements a html template benchmark,
//! creating a dynamic html output from a template and the given input.
//!
//! Adapted from:
//! SeBS: Serverless Benchmark Suite
//! https://github.com/spcl/serverless-benchmarks/tree/master/benchmarks/100.webapps/110.dynamic-html/nodejs

use std::cmp::min;
use askama::Template;
use bench::*;
use random_number::random;
use std::env;

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_template NUMBER");
            return;
        }

        match args[0].parse::<usize>() {
            Err(e) => eprintln!("Error: {}", e),
            Ok(n) => {
                let generated = (0..n).map(|_| random!(0, 100_000)).collect::<Vec<_>>();
               
                let tmpl = DataTemplate { n, generated };
                match tmpl.render() {
                    Err(e) => eprintln!("Error: {}", e),
                    Ok(s) => {
                        println!("{} ...", &s[..min(800, s.len())]);
                    },
                }
            }
        }
    });
}

#[derive(Template)]
#[template(
    source = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Randomly generated data.</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link
      href="http://netdna.bootstrapcdn.com/bootstrap/3.0.0/css/bootstrap.min.css"
      rel="stylesheet" media="screen">
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
</html>
"#,
    ext = "html"
)]
struct DataTemplate {
    n: usize,
    generated: Vec<u64>,
}
