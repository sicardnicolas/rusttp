mod request {
  struct RequestParser {
    command: String,
  }

  impl RequestParser {
    pub fn parse(&self) -> Request {
      let lines = self.command.split("\r\n");

      if 0 == lines.len() {
        Request::new()
      }

      let elements = lines[0].split(" ");

      if 3 != elements.len() {
        Request::new()
      }

      return Request {
        method: set_method(elements[0]),
        path: set_path(elements[1]),
        protocal_version: set_protocol_version(elements[2]),
        headers: set_headers(lines[1..]),
        query_params: set_query_params(elements[1])
      }
    }
  }

  fn set_method(method: String) -> String {

  }

  fn set_protocol_version(protocol_version: String) -> String {

  }

  fn set_path(path: String) -> String {

  }

  fn set_headers(lines: Vec<String>) -> HashMap<str, str> {

  }

  fn set_query_params(path: String) -> HashMap<str, str> {

  }

  fn get_http_verbs() -> [&str; 8] {
    ["GET", "HEAD", "POST", "OPTIONS", "TRACE", "PUT", "PATCH", "DELETE"]
  }

  fn get_http_version() -> [&str; 3] {
    ["1.0", "1.1", "2"]
  }
}
