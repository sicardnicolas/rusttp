use bytes::Bytes;
use std::collections::HashMap;

mod response {
    struct Response {
        protocol_version: String,
        status_code: i32,
        headers: HashMap<str, str>,
        body: HashMap<str, str>,
    }

    impl Response {
        pub fn format(&self) -> Bytes {
            let status_message = self.get_message_for_status_code();
            let mut response_content: &str =
                self.protocol_version + " " + self.status_code.to_string() + " " + status_message;

            let header_lines = self.get_headers_as_lines();
            for header_line in header_lines {
                response_content.push_str("\r\n" + header_line);
            }

            if 0 < self.body.len() {
                response_content.push_str("\r\nContent-Length: " + self.body.len());
                response_content.push_str("\r\n\r\n" + self.body);
            }

            response_content.as_bytes()
        }

        fn get_headers_as_lines(&self) -> Vec<&str> {
            let mut header_lines: Vec<&str> = Vec::new();

            for (key, value) in self.headers.into_iter() {
                if "Content-Length" == key {
                    continue;
                }
                header_lines.push(key + ": " + value)
            }

            header_lines
        }

        fn get_message_for_status_code(&self) -> &str {
            match self.status_code {
                0 => "Unassigned",
                100 => "cont",
                101 => "switching_protocols",
                102 => "processing",
                103 => "checkpoint_draft",
                200 => "OK",
                201 => "Created",
                202 => "accepted",
                203 => "non_authoritative_information",
                204 => "no_content",
                205 => "reset_content",
                206 => "partial_content",
                207 => "multi_status",
                208 => "already_reported",
                226 => "im_used",
                300 => "multiple_choices",
                301 => "moved_permanently",
                302 => "found",
                303 => "see_other",
                304 => "not_modified",
                305 => "use_proxy",
                306 => "switch_proxy",
                307 => "temporary_redirect",
                308 => "permanent_redirect",
                400 => "bad_request",
                401 => "unauthorized",
                402 => "payment_required",
                403 => "forbidden",
                404 => "not_found",
                405 => "method_not_allowed",
                406 => "not_acceptable",
                407 => "proxy_authentication_required",
                408 => "request_timeout",
                409 => "conflict",
                410 => "gone",
                411 => "length_required",
                412 => "precondition_failed",
                413 => "request_entity_too_large",
                414 => "request_uri_too_long",
                415 => "unsupported_media_type",
                416 => "requested_range_not_satisfiable",
                417 => "expectation_failed",
                418 => "im_a_teapot",
                421 => "misdirected_request",
                422 => "unprocessable_entity",
                423 => "locked",
                424 => "failed_dependency",
                425 => "unordered_collection",
                426 => "upgrade_required",
                428 => "precondition_required",
                429 => "too_many_requests",
                431 => "request_header_fields_too_large",
                451 => "unavailable_for_legal_reasons",
                499 => "client_closed_request",
                500 => "internal_server_error",
                501 => "not_implemented",
                502 => "bad_gateway",
                503 => "service_unavailable",
                504 => "gateway_timeout",
                505 => "http_version_not_supported",
                506 => "variant_also_negotiates",
                507 => "insufficient_storage",
                508 => "loop_detected",
                509 => "bandwidth_limit_exceeded",
                510 => "not_extended",
                511 => "network_authentication_required",
                _ => "Status code undefined",
            }
        }
    }
}
