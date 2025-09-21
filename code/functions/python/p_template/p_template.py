import random
import sys

from jinja2 import Template


def main():
    """
    Implements a html template benchmark,
    creating a dynamic html output from a template and the given input.

    Adapted from:
    SeBS: Serverless Benchmark Suite
    https://github.com/spcl/serverless-benchmarks/blob/master/benchmarks/100.webapps/110.dynamic-html/python/function.py
    """
    args = sys.argv[1:]
    if len(args) < 1:
        print("Pass a number as argument.", file=sys.stderr)
        return
    try:
        n = int(args[0])
    except ValueError:
        print("Pass a number as argument.", file=sys.stderr)
        return

    generated_data = [random.choice(range(0, 100_000)) for _ in range(n)]
    res = render_template(n, generated_data)
    print(f"{res[:800]} ...")


def render_template(n, generated):
    template = Template("""
    <!DOCTYPE html>
    <html>
    <head>
        <title>Randomly generated data.</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link href="http://netdna.bootstrapcdn.com/bootstrap/3.0.0/css/bootstrap.min.css" rel="stylesheet" media="screen">
        <style type="text/css">
            .container {
                max-width: 500px;
                padding-top: 100px;
            }
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
    """)

    return template.render(n=n, generated=generated)
