use rustler::types::atom::{error, ok};
use rustler::types::tuple::make_tuple;
use rustler::{Encoder, Env, Error, Term};

mod lint_html;

#[rustler::nif]
fn lint(env: Env, input: String) -> Result<Term, Error> {
    let parse_errors = lint_html::lint(input);
    convert_errors(env, parse_errors)
}

fn convert_errors(env: Env, parse_errors: Vec<(u64, String)>) -> Result<Term, Error> {
    if parse_errors.len() == 0 {
        Ok(ok().to_term(env))
    } else {
        let e = error().encode(env);
        let msgs: Vec<Term> = parse_errors.iter().map(|x| x.encode(env)).collect();
        let msgs_term: Term = msgs.encode(env);
        let tuple_elems = vec![e, msgs_term];
        Ok(make_tuple(env, &tuple_elems))
    }
}

rustler::init!("Elixir.ExHtml5ever.Native", [lint]);
