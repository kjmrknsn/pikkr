use farmhash_collections::FarmHashMap;
use std::cmp;
use error::{Error, ErrorKind};
use result::Result;
use utf8::{DOLLAR, DOT};

const ROOT_QUERY_STR_OFFSET: usize = 2;


#[derive(Debug)]
pub struct Query<'a> {
    pub i: usize,
    pub ri: usize,
    pub target: bool,
    pub children: Option<FarmHashMap<&'a [u8], Query<'a>>>,
}

/// A pattern tree associated with the queries
pub struct QueryTree<'a> {
    pub root: FarmHashMap<&'a [u8], Query<'a>>,
    pub num_queries: usize,
    pub max_depth: usize,
    pub num_nodes: usize,
}

impl<'a> QueryTree<'a> {
    pub fn new<S: ?Sized + AsRef<[u8]>>(queries: &[&'a S]) -> Result<Self> {
        let mut root = FarmHashMap::default();
        let mut level = 0;
        let mut qi = 0;

        for (ri, s) in queries.into_iter().map(|s| (*s).as_ref()).enumerate() {
            if !is_valid_query_str(s) {
                return Err(Error::from(ErrorKind::InvalidQuery));
            }

            let l = set_queries(&mut root, &s[ROOT_QUERY_STR_OFFSET..], || {
                let query = Query {
                    i: qi,
                    ri,
                    target: false,
                    children: None,
                };
                qi += 1;
                query
            });

            level = cmp::max(level, l);
        }

        Ok(Self {
            root,
            num_queries: queries.len(),
            max_depth: level,
            num_nodes: qi,
        })
    }
}

#[inline]
fn is_valid_query_str(query_str: &[u8]) -> bool {
    if query_str.len() < ROOT_QUERY_STR_OFFSET + 1 || query_str[0] != DOLLAR || query_str[1] != DOT {
        return false;
    }
    let mut s = ROOT_QUERY_STR_OFFSET - 1;
    for i in s + 1..query_str.len() {
        if query_str[i] != DOT {
            continue;
        }
        if i == s + 1 || i == query_str.len() - 1 {
            return false;
        }
        s = i;
    }
    true
}

#[inline]
fn set_queries<'a, F>(queries: &mut FarmHashMap<&'a [u8], Query<'a>>, s: &'a [u8], mut factory: F) -> usize
where
    F: FnMut() -> Query<'a>,
{
    let j = s.iter().position(|&c| c == DOT).unwrap_or(s.len());
    let (t, u) = s.split_at(j);

    let query = queries.entry(t).or_insert_with(&mut factory);
    if u.len() > 1 {
        // The remaining segments are exist
        let children = query.children.get_or_insert(FarmHashMap::default());
        set_queries(children, &u[1..], factory) + 1
    } else {
        query.target = true; // mark the node as a target node
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_query_str() {
        struct TestCase<'a> {
            query_str: &'a str,
            want: bool,
        }
        let test_cases = vec![
            TestCase {
                query_str: "",
                want: false,
            },
            TestCase {
                query_str: "$",
                want: false,
            },
            TestCase {
                query_str: "$.",
                want: false,
            },
            TestCase {
                query_str: "$..",
                want: false,
            },
            TestCase {
                query_str: "a.a",
                want: false,
            },
            TestCase {
                query_str: "$aa",
                want: false,
            },
            TestCase {
                query_str: "$.a",
                want: true,
            },
            TestCase {
                query_str: "$.aaaa",
                want: true,
            },
            TestCase {
                query_str: "$.aaaa.",
                want: false,
            },
            TestCase {
                query_str: "$.aaaa.b",
                want: true,
            },
            TestCase {
                query_str: "$.aaaa.bbbb",
                want: true,
            },
            TestCase {
                query_str: "$.aaaa.bbbb.",
                want: false,
            },
        ];
        for t in test_cases {
            let got = is_valid_query_str(t.query_str.as_bytes());
            assert_eq!(t.want, got);
        }
    }
}
