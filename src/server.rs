use std::sync::{Arc, RwLock};

use varlink::{VarlinkService};
use super::org_wordtree_ngrams;
use super::org_wordtree_ngrams::{VarlinkInterface, Call_Ping, Call_Lookup, Call_StopServing};
use super::common::{Tally};

struct WordtreeNgrams {
    tally: Arc<RwLock<Tally>>
}

impl VarlinkInterface for WordtreeNgrams {
    fn ping(&self, call: &mut Call_Ping, ping: String) -> varlink::Result<()> {
        eprintln!("Ping received.");
        call.reply(ping)
    }

    fn lookup(&self, call: &mut Call_Lookup, ngram: String) -> varlink::Result<()> {
        // eprintln!("Lookup called.");
        let tally = self.tally.read().unwrap();
        call.reply(match tally.get(&ngram) {
            Some(value) => *value,
            None => 0
        })
    }

    fn stop_serving(&self, call: &mut Call_StopServing) -> varlink::Result<()> {
        call.reply()?;
        Err(varlink::ErrorKind::ConnectionClosed.into())
    }
}

pub fn run_server(address: &str, tally: Arc<RwLock<Tally>>) -> varlink::Result<()> {
    let wn = WordtreeNgrams { tally };
    let myinterface = org_wordtree_ngrams::new(Box::new(wn));
    let service = VarlinkService::new(
        "org.wordtree",
        "ngrams",
        "0.1",
        "http://wordtree.org",
        vec![Box::new(myinterface)],
    );
    varlink::listen(service, &address, 1, 10, 0)?;
    Ok(())
}