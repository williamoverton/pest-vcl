# pest-vcl
Parse VCL into an abstract syntax tree.

## Introduction

This project is not intended for any serious use and is just a hobby project! 
The tool can be used for parsing VCL and outputting a JSON object containing parsed the logic.

Example Input
```
sub vcl_recv {
  set req.http.MyHeader = "Hello World!";
}
```

Example Output:
`cargo run -- --file examples/basic.vcl`
```json
[
  {
    "name": "vcl_recv",
    "statements": [
      {
        "assign_operator": "=",
        "assignee": "req.http.MyHeader",
        "type": "set_exp",
        "value": {
          "type": "string",
          "value": "\"Hello World!\""
        }
      }
    ],
    "type": "sub"
  }
]
```

### Usage:
```
USAGE:
    pest-vcl --file <FILE>

For more information try --help
```
