use std::collections::HashMap;

mod request {
	struct Request {
		method: String,
		path: String,
		protocol_version: String,
		headers: HashMap<str, str>,
		query_params: HashMap<str, str>,
	}

	impl Request {
		pub fn get_method(&self) -> String {
			self.method
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn test_get_method() {
			let r = Request {
				method: String::new("GET"),
				path: String::new("/"),
				protocol_version: String::new("HTTP/2"),
				headers: HashMap::new(),
				query_params: HashMap::new()
			};

			assert_eq!("GET", r.get_method());
		}
	}
}
