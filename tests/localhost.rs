use ssh::*;

#[test]
fn main() {
    {
        let mut session = github_session();
        session.connect().unwrap();
        assert_maybe_known(&mut session);
    }
    {
        let mut session = github_session();
        let mut session2 = github_session();
        session.connect().unwrap();
        session2.connect().unwrap();
        assert_maybe_known(&mut session);
        assert_maybe_known(&mut session2);
    }
}

fn github_session() -> Session {
    let mut session = Session::new().unwrap();
    session.set_host("github.com").unwrap();
    session
}

fn assert_maybe_known(session: &mut Session) {
    let known = session.is_server_known().unwrap();
    assert!(known == ServerKnown::Known || known == ServerKnown::NotKnown);
}
