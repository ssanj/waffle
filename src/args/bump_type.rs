use crate::error::{WaffleError, ResultW};

pub enum BumpType {
  Major,
  Minor,
  Patch
}


impl BumpType {

  pub fn get_bump_type(major: bool, minor: bool, patch: bool) -> ResultW<BumpType> {
      let set_flags =
        vec![major, minor, patch]
          .into_iter()
          .filter(|v| *v)
          .count();

      if set_flags > 1 {
        Err(WaffleError::InvalidBumpCombination)
      } else {
        let bump_type = {
          if major {
            BumpType::Major
          } else if minor {
            BumpType::Minor
          } else {
            BumpType::Patch
          }
        };

        Ok(bump_type)
      }
  }
}
