#![doc = "This file was automatically generated by the varlink rust generator"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::BufRead;
use std::sync::{Arc, RwLock};
use varlink::{self, CallTrait};
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorKind {
    Varlink_Error,
    VarlinkReply_Error,
    NgramsError(Option<NgramsError_Args>),
}
impl ::std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ErrorKind::Varlink_Error => write!(f, "Varlink Error"),
            ErrorKind::VarlinkReply_Error => write!(f, "Varlink error reply"),
            ErrorKind::NgramsError(v) => write!(f, "org.wordtree.ngrams.NgramsError: {:#?}", v),
        }
    }
}
pub struct Error(
    pub ErrorKind,
    pub Option<Box<dyn std::error::Error + 'static>>,
    pub Option<&'static str>,
);
impl Error {
    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}
impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error(e, None, None)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.1.as_ref().map(|e| e.as_ref())
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error as StdError;
        if let Some(ref o) = self.2 {
            std::fmt::Display::fmt(o, f)?;
        }
        std::fmt::Debug::fmt(&self.0, f)?;
        if let Some(e) = self.source() {
            std::fmt::Display::fmt("\nCaused by:\n", f)?;
            std::fmt::Debug::fmt(&e, f)?;
        }
        Ok(())
    }
}
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;
impl From<varlink::Error> for Error {
    fn from(e: varlink::Error) -> Self {
        match e.kind() {
            varlink::ErrorKind::VarlinkErrorReply(r) => Error(
                ErrorKind::from(r),
                Some(Box::from(e)),
                Some(concat!(file!(), ":", line!(), ": ")),
            ),
            _ => Error(
                ErrorKind::Varlink_Error,
                Some(Box::from(e)),
                Some(concat!(file!(), ":", line!(), ": ")),
            ),
        }
    }
}
#[allow(dead_code)]
impl Error {
    pub fn source_varlink_kind(&self) -> Option<&varlink::ErrorKind> {
        use std::error::Error as StdError;
        let mut s: &StdError = self;
        while let Some(c) = s.source() {
            let k = self
                .source()
                .and_then(|e| e.downcast_ref::<varlink::Error>())
                .and_then(|e| Some(e.kind()));
            if k.is_some() {
                return k;
            }
            s = c;
        }
        None
    }
}
impl From<&varlink::Reply> for ErrorKind {
    #[allow(unused_variables)]
    fn from(e: &varlink::Reply) -> Self {
        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.wordtree.ngrams.NgramsError" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p.clone()) {
                    Ok(v) => ErrorKind::NgramsError(v),
                    Err(_) => ErrorKind::NgramsError(None),
                },
                _ => ErrorKind::NgramsError(None),
            },
            _ => ErrorKind::VarlinkReply_Error,
        }
    }
}
pub trait VarlinkCallError: varlink::CallTrait {
    fn reply_ngrams_error(&mut self, r#reason: String) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.wordtree.ngrams.NgramsError",
            Some(
                serde_json::to_value(NgramsError_Args { r#reason })
                    .map_err(varlink::map_context!())?,
            ),
        ))
    }
}
impl<'a> VarlinkCallError for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NgramsError_Args {
    pub r#reason: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Lookup_Reply {
    pub r#tally: i64,
}
impl varlink::VarlinkReply for Lookup_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Lookup_Args {
    pub r#ngram: String,
}
pub trait Call_Lookup: VarlinkCallError {
    fn reply(&mut self, r#tally: i64) -> varlink::Result<()> {
        self.reply_struct(Lookup_Reply { r#tally }.into())
    }
}
impl<'a> Call_Lookup for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ping_Reply {
    pub r#pong: String,
}
impl varlink::VarlinkReply for Ping_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ping_Args {
    pub r#ping: String,
}
pub trait Call_Ping: VarlinkCallError {
    fn reply(&mut self, r#pong: String) -> varlink::Result<()> {
        self.reply_struct(Ping_Reply { r#pong }.into())
    }
}
impl<'a> Call_Ping for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopServing_Reply {}
impl varlink::VarlinkReply for StopServing_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopServing_Args {}
pub trait Call_StopServing: VarlinkCallError {
    fn reply(&mut self) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}
impl<'a> Call_StopServing for varlink::Call<'a> {}
pub trait VarlinkInterface {
    fn lookup(&self, call: &mut Call_Lookup, r#ngram: String) -> varlink::Result<()>;
    fn ping(&self, call: &mut Call_Ping, r#ping: String) -> varlink::Result<()>;
    fn stop_serving(&self, call: &mut Call_StopServing) -> varlink::Result<()>;
    fn call_upgraded(
        &self,
        _call: &mut varlink::Call,
        _bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
pub trait VarlinkClientInterface {
    fn lookup(&mut self, r#ngram: String) -> varlink::MethodCall<Lookup_Args, Lookup_Reply, Error>;
    fn ping(&mut self, r#ping: String) -> varlink::MethodCall<Ping_Args, Ping_Reply, Error>;
    fn stop_serving(&mut self) -> varlink::MethodCall<StopServing_Args, StopServing_Reply, Error>;
}
#[allow(dead_code)]
pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
}
impl VarlinkClient {
    #[allow(dead_code)]
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient { connection }
    }
}
impl VarlinkClientInterface for VarlinkClient {
    fn lookup(&mut self, r#ngram: String) -> varlink::MethodCall<Lookup_Args, Lookup_Reply, Error> {
        varlink::MethodCall::<Lookup_Args, Lookup_Reply, Error>::new(
            self.connection.clone(),
            "org.wordtree.ngrams.Lookup",
            Lookup_Args { r#ngram },
        )
    }
    fn ping(&mut self, r#ping: String) -> varlink::MethodCall<Ping_Args, Ping_Reply, Error> {
        varlink::MethodCall::<Ping_Args, Ping_Reply, Error>::new(
            self.connection.clone(),
            "org.wordtree.ngrams.Ping",
            Ping_Args { r#ping },
        )
    }
    fn stop_serving(&mut self) -> varlink::MethodCall<StopServing_Args, StopServing_Reply, Error> {
        varlink::MethodCall::<StopServing_Args, StopServing_Reply, Error>::new(
            self.connection.clone(),
            "org.wordtree.ngrams.StopServing",
            StopServing_Args {},
        )
    }
}
#[allow(dead_code)]
pub struct VarlinkInterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}
#[allow(dead_code)]
pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> VarlinkInterfaceProxy {
    VarlinkInterfaceProxy { inner }
}
impl varlink::Interface for VarlinkInterfaceProxy {
    fn get_description(&self) -> &'static str {
        "interface org.wordtree.ngrams\n\n# Returns the same string\nmethod Ping(ping: string) -> (pong: string)\n\n# Looks up an ngram and returns its tally\nmethod Lookup(ngram: string) -> (tally: int)\n\n# Stop serving\nmethod StopServing() -> ()\n\n# Something failed\nerror NgramsError (reason: string)\n"
    }
    fn get_name(&self) -> &'static str {
        "org.wordtree.ngrams"
    }
    fn call_upgraded(
        &self,
        call: &mut varlink::Call,
        bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        self.inner.call_upgraded(call, bufreader)
    }
    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.wordtree.ngrams.Lookup" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Lookup_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner.lookup(call as &mut Call_Lookup, args.r#ngram)
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "org.wordtree.ngrams.Ping" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Ping_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es)).into()
                            );
                        }
                    };
                    self.inner.ping(call as &mut Call_Ping, args.r#ping)
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "org.wordtree.ngrams.StopServing" => {
                self.inner.stop_serving(call as &mut Call_StopServing)
            }
            m => call.reply_method_not_found(String::from(m)),
        }
    }
}