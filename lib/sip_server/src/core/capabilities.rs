pub use crate::{Error, SipManager};
use models::transport::{RequestMsg, ResponseMsg};
use rsip::common::uri::HostWithPort;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct Capabilities {
    sip_manager: Weak<SipManager>,
}

#[allow(clippy::new_without_default)]
impl Capabilities {
    pub fn new(sip_manager: Weak<SipManager>) -> Self {
        Self { sip_manager }
    }

    pub async fn process_incoming_request(&self, msg: RequestMsg) -> Result<(), Error> {
        apply_default_checks(&msg.sip_request)?;

        let response = create_busy_here_from(msg.sip_request.clone())?;

        self.sip_manager()
            .transport
            .send(ResponseMsg::from((response, msg.peer, msg.transport)).into())
            .await
    }

    fn sip_manager(&self) -> Arc<SipManager> {
        self.sip_manager.upgrade().expect("sip manager is missing!")
    }
}

fn apply_default_checks(request: &rsip::Request) -> Result<(), Error> {
    has_correct_request_uri(&request.uri)?;

    Ok(())
}

fn has_correct_request_uri(request_uri: &rsip::common::Uri) -> Result<(), Error> {
    if request_uri.host_with_port == default_request_uri() {
        Ok(())
    } else {
        Err(Error::from("invalid request uri"))
    }
}

fn default_request_uri() -> HostWithPort {
    HostWithPort::SocketAddr(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        5060,
    ))
}

fn create_busy_here_from(request: rsip::Request) -> Result<rsip::Response, crate::Error> {
    use rsip::{
        common::{ContentType, Language, Method},
        headers::*,
        message::HeadersExt,
        Headers,
    };

    if *request.method() != Method::Options {
        return Err(crate::Error::custom(format!(
            "must have OPTIONS method, found: {}",
            request.method()
        )));
    }

    let mut headers: Headers = Default::default();

    let mut to = request.to_header()?.clone();

    headers.push(request.via_header()?.clone().into());
    to.0.add_param(NamedParam::Tag(Default::default()));
    headers.push(to.into());

    headers.push(request.from_header()?.clone().into());
    headers.push(request.call_id_header()?.clone().into());
    headers.push(request.cseq_header()?.clone().into());
    headers.push(Header::ContentLength(Default::default()));
    headers.push(Header::Server(Default::default()));
    headers.push(Allow(Method::all()).into());
    headers.push(Accept("application/sdp".into()).into());
    headers.push(AcceptEncoding(ContentType::Custom("gzip".into())).into());
    headers.push(AcceptLanguage(Language::English).into());

    Ok(rsip::Response {
        code: 486.into(),
        headers,
        ..Default::default()
    })
}