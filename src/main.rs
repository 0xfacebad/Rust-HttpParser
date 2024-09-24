use std::collections::HashMap;
#[derive(Debug)]
#[allow(dead_code)]
struct HttpRequest{
       method:String,
       path:String,
       version:String,
       headers:HashMap<String,String>,
       body:String,
}
 fn parse_http_req(req:&str)->HttpRequest{
             let mut lines = req.lines();
             let request_line = lines.next().unwrap();
             let part:Vec<&str> = request_line.split_whitespace().collect();
             let method = part[0].to_string();
             let path = part[1].to_string();
             let version = part[2].to_string();

             let mut headers = HashMap::new();
             for line  in lines.clone().into_iter(){
                     if line.is_empty(){
                            break;
                     }
                     let headers_parts:Vec<&str> = line.split(": ").collect();
                      headers.insert(headers_parts[0].to_string() , headers_parts[1].to_string());
             }

             let body = lines.collect::<Vec<&str>>().join("\n");
             HttpRequest {method , path ,version , headers , body}
      }
fn main() {
     let req = "GET /index.html HTTP/1.1\r\n\
                       Host: www.example.com\r\n\
                       User-Agent: curl/7.68.0\r\n\
                       Accept: */*\r\n\r\n\
                       This is the body of the request.";

      let some_parse = parse_http_req(req);
      println!("{:#?}" , some_parse);
}
