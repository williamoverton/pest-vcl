sub vcl_recv {
  set req.http.MyHeader = "Hello World!";
}