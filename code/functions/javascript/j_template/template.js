import mustache from 'mustache';

/**
 * Implements a html template benchmark,
 * creating a dynamic html output from a template and the given input.
 *
 * Adapted from:
 * SeBS: Serverless Benchmark Suite
 * https://github.com/spcl/serverless-benchmarks/tree/master/benchmarks/100.webapps/110.dynamic-html/nodejs
 */
export function main(n) {
    const generated_data = Array.from({length: n}, () => random(0, 100_000));
    const data = {
        n: n,
        generated: generated_data
    };
    const result = mustache.render(template, data);

    console.log(result.slice(0, 800))
}

/**
 * Generates a random integer between b and e.
 * @param b inclusive lower bound
 * @param e inclusive upper bound
 * @returns {number}
 */
function random(b, e) {
    return Math.round(Math.random() * (e - b) + b);
}

const template = `
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
      <p>Requested: {{n}}</p>
      <p>Requested random numbers:</p>
      <ul>
        {{#generated}}
        <li>{{.}}</li>
        {{/generated}}
      </ul>
    </div>
  </body>
</html>
`;
