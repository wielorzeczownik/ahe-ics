use std::fmt;
use std::net::IpAddr;

use axum::http::HeaderMap;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ClientIpSource {
  PeerAddr,
  Header,
  HeaderInvalid,
}

impl fmt::Display for ClientIpSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let label = match self {
      Self::PeerAddr => "peer_addr",
      Self::Header => "header",
      Self::HeaderInvalid => "invalid_header",
    };
    f.write_str(label)
  }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ClientIp {
  pub(crate) ip: IpAddr,
  pub(crate) source: ClientIpSource,
}

pub(crate) fn resolve_client_ip(
  peer_ip: IpAddr,
  headers: &HeaderMap,
  real_ip_header: Option<&str>,
) -> ClientIp {
  let Some(header_name) = real_ip_header else {
    return ClientIp {
      ip: peer_ip,
      source: ClientIpSource::PeerAddr,
    };
  };

  let Some(value) = headers.get(header_name) else {
    return ClientIp {
      ip: peer_ip,
      source: ClientIpSource::PeerAddr,
    };
  };

  let Ok(value) = value.to_str() else {
    return ClientIp {
      ip: peer_ip,
      source: ClientIpSource::HeaderInvalid,
    };
  };

  let Some(ip) = parse_forwarded_ip(value) else {
    return ClientIp {
      ip: peer_ip,
      source: ClientIpSource::HeaderInvalid,
    };
  };

  ClientIp {
    ip,
    source: ClientIpSource::Header,
  }
}

fn parse_forwarded_ip(value: &str) -> Option<IpAddr> {
  let first = value.split(',').next()?.trim();
  if first.is_empty() {
    return None;
  }

  let mut candidate = first.trim_matches('"');
  if let Some(stripped) = candidate
    .strip_prefix("for=")
    .or_else(|| candidate.strip_prefix("For="))
  {
    candidate = stripped.trim();
  }

  candidate.parse().ok()
}

#[cfg(test)]
mod tests {
  use axum::http::{HeaderName, HeaderValue};

  use super::*;

  const PEER: &str = "10.0.0.1";

  fn peer_ip() -> IpAddr {
    PEER.parse().expect("valid peer ip")
  }

  fn headers_with(name: &str, value: &[u8]) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
      HeaderName::from_bytes(name.as_bytes()).expect("valid header name"),
      HeaderValue::from_bytes(value).expect("valid header value"),
    );
    headers
  }

  #[test]
  fn parses_plain_addresses() {
    assert_eq!(
      parse_forwarded_ip("203.0.113.7"),
      Some("203.0.113.7".parse().expect("valid ip"))
    );
    assert_eq!(
      parse_forwarded_ip("2001:db8::1"),
      Some("2001:db8::1".parse().expect("valid ip"))
    );
  }

  #[test]
  fn takes_first_hop_of_forwarded_chain() {
    assert_eq!(
      parse_forwarded_ip(" 203.0.113.7 , 70.41.3.18 , 150.172.238.178 "),
      Some("203.0.113.7".parse().expect("valid ip"))
    );
  }

  #[test]
  fn unwraps_quoted_and_forwarded_syntax() {
    let expected: IpAddr = "203.0.113.7".parse().expect("valid ip");

    assert_eq!(parse_forwarded_ip("\"203.0.113.7\""), Some(expected));
    assert_eq!(parse_forwarded_ip("for=203.0.113.7"), Some(expected));
    assert_eq!(parse_forwarded_ip("For= 203.0.113.7"), Some(expected));
    assert_eq!(parse_forwarded_ip("\"for=203.0.113.7\""), Some(expected));
  }

  #[test]
  fn rejects_unparsable_values() {
    assert_eq!(parse_forwarded_ip(""), None);
    assert_eq!(parse_forwarded_ip("   "), None);
    assert_eq!(parse_forwarded_ip(", 203.0.113.7"), None);
    assert_eq!(parse_forwarded_ip("not-an-ip"), None);
    // A port suffix is not stripped, so the value fails to parse
    assert_eq!(parse_forwarded_ip("203.0.113.7:1234"), None);
  }

  #[test]
  fn falls_back_to_peer_when_header_is_not_configured() {
    let headers = headers_with("x-forwarded-for", b"203.0.113.7");
    let resolved = resolve_client_ip(peer_ip(), &headers, None);

    assert_eq!(resolved.ip, peer_ip());
    assert!(matches!(resolved.source, ClientIpSource::PeerAddr));
  }

  #[test]
  fn falls_back_to_peer_when_header_is_absent() {
    let resolved = resolve_client_ip(peer_ip(), &HeaderMap::new(), Some("x-forwarded-for"));

    assert_eq!(resolved.ip, peer_ip());
    assert!(matches!(resolved.source, ClientIpSource::PeerAddr));
  }

  #[test]
  fn uses_header_address_when_present() {
    let headers = headers_with("x-forwarded-for", b"203.0.113.7, 70.41.3.18");
    let resolved = resolve_client_ip(peer_ip(), &headers, Some("x-forwarded-for"));
    let expected: IpAddr = "203.0.113.7".parse().expect("valid ip");

    assert_eq!(resolved.ip, expected);
    assert!(matches!(resolved.source, ClientIpSource::Header));
  }

  #[test]
  fn reports_invalid_header_but_keeps_peer_address() {
    let headers = headers_with("x-forwarded-for", b"not-an-ip");
    let resolved = resolve_client_ip(peer_ip(), &headers, Some("x-forwarded-for"));

    assert_eq!(resolved.ip, peer_ip());
    assert!(matches!(resolved.source, ClientIpSource::HeaderInvalid));
  }

  #[test]
  fn reports_invalid_header_for_non_ascii_bytes() {
    let headers = headers_with("x-forwarded-for", &[0xff, 0xfe]);
    let resolved = resolve_client_ip(peer_ip(), &headers, Some("x-forwarded-for"));

    assert_eq!(resolved.ip, peer_ip());
    assert!(matches!(resolved.source, ClientIpSource::HeaderInvalid));
  }

  #[test]
  fn source_labels_are_stable() {
    assert_eq!(ClientIpSource::PeerAddr.to_string(), "peer_addr");
    assert_eq!(ClientIpSource::Header.to_string(), "header");
    assert_eq!(ClientIpSource::HeaderInvalid.to_string(), "invalid_header");
  }
}
