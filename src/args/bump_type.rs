use crate::error::{WaffleError, ResultW};

#[derive(Debug, PartialEq)]
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

      if set_flags == 0 {
        Err(WaffleError::NoBumpCombinations)
      } else if set_flags > 1 {
        Err(WaffleError::TooManyBumpCombinations)
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_bump_type_with_patch() {
        let bump_type = BumpType::get_bump_type(false, false, true).unwrap();
        let expected_bump_type = BumpType::Patch;

        assert_eq!(bump_type, expected_bump_type)
    }


    #[test]
    fn get_bump_type_with_minor() {
        let bump_type = BumpType::get_bump_type(false, true, false).unwrap();
        let expected_bump_type = BumpType::Minor;

        assert_eq!(bump_type, expected_bump_type)
    }


    #[test]
    fn get_bump_type_with_major() {
        let bump_type = BumpType::get_bump_type(true, false, false).unwrap();
        let expected_bump_type = BumpType::Major;

        assert_eq!(bump_type, expected_bump_type)
    }


    #[test]
    fn get_bump_type_with_more_than_one_type() {
        let expected_bump_type_error = Err(WaffleError::TooManyBumpCombinations);

        let bump_type_major_minor = BumpType::get_bump_type(true, true, false);
        let bump_type_minor_patch = BumpType::get_bump_type(false, true, true);
        let bump_type_major_patch = BumpType::get_bump_type(true, false, true);
        let bump_type_major_minor_patch = BumpType::get_bump_type(true, true, true);

        assert_eq!(bump_type_major_minor, expected_bump_type_error);
        assert_eq!(bump_type_minor_patch, expected_bump_type_error);
        assert_eq!(bump_type_major_patch, expected_bump_type_error);
        assert_eq!(bump_type_major_minor_patch, expected_bump_type_error)
    }


    #[test]
    fn get_bump_type_with_no_types() {
        let bump_type = BumpType::get_bump_type(false, false, false);
        let expected_bump_type_error = Err(WaffleError::NoBumpCombinations);

        assert_eq!(bump_type, expected_bump_type_error)
    }
}
