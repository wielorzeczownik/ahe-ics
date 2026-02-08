use axum::http::HeaderMap;
use std::fmt;
use std::net::IpAddr;

use crate::config::Config;

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

pub(crate) fn resolve_client_ip(peer_ip: IpAddr, headers: &HeaderMap, config: &Config) -> ClientIp {
  let Some(header_name) = config.real_ip_header.as_deref() else {
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
  if let Some(stripped) = candidate.strip_prefix("for=") {
    candidate = stripped.trim();
  } else if let Some(stripped) = candidate.strip_prefix("For=") {
    candidate = stripped.trim();
  }

  candidate.parse().ok()
}
