use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    check::{CheckError, Checker, Checkers, Flatten, IntoFlat, Notice},
    value::{Value, ValueKind},
};

pub trait CheckList {
    fn commit(&self, key: &str, value: Value) -> Result<Option<Commit>, CheckError>;
    fn items(&self) -> HashMap<String, Vec<ValueKind>>;
}

impl CheckList for HashMap<String, Flatten<Checkers>> {
    fn commit(&self, key: &str, value: Value) -> Result<Option<Commit>, CheckError> {
        let Some(n) = self.get(key) else {
            return Ok(None);
        };
        let notice = n.check(&value)?;
        Ok(Some(Commit {
            key: key.to_string(),
            value,
            notice,
        }))
    }

    fn items(&self) -> HashMap<String, Vec<ValueKind>> {
        self.iter()
            .map(|item| (item.0.to_string(), item.1.expecting()))
            .collect()
    }
}

pub trait IntoCheckList {
    fn into_checklist(self) -> Result<impl CheckList, String>;
}

impl IntoCheckList for Vec<(String, Checkers)> {
    fn into_checklist(self) -> Result<impl CheckList, String> {
        let mut hashmap = HashMap::new();
        for (k, v) in &self.into_iter().group_by(|x| x.0.to_string()) {
            hashmap.insert(k, v.map(|x| x.1).into_flat()?);
        }
        Ok(hashmap)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Commit {
    key: String,
    value: Value,
    notice: Notice,
}

#[cfg(test)]
mod tests {
    use crate::{
        check::{Checkers, Notice},
        commit::CheckList,
    };

    use super::IntoCheckList;

    #[test]
    fn test() {
        let lists = vec![
            (
                "A".to_string(),
                Checkers::Exact("abc".to_string(), "caution".to_string()),
            ),
            (
                "B".to_string(),
                Checkers::Between(-5.0, 5.0, "error".to_string()),
            ),
            (
                "B".to_string(),
                Checkers::Between(-2.0, 2.0, "caution".to_string()),
            ),
        ];
        let map = lists.into_checklist().unwrap();
        assert_eq!(
            map.commit("A", "abc".into()).unwrap().unwrap().notice,
            Notice::Clear
        );
        assert_eq!(
            map.commit("A", "abcd".into()).unwrap().unwrap().notice,
            Notice::Attention("caution".to_string())
        );
        assert_eq!(
            map.commit("B", 0.into()).unwrap().unwrap().notice,
            Notice::Clear
        );
        assert_eq!(
            map.commit("B", 3.into()).unwrap().unwrap().notice,
            Notice::Attention("caution".to_string())
        );
        assert_eq!(
            map.commit("B", 6.into()).unwrap().unwrap().notice,
            Notice::Attention("error".to_string())
        );
    }
}
