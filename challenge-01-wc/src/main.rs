use std::{fs::File, io::Read};

const TEST_FILE_PATH: &str= "/Users/voxelost/workspace/dev/codingchallenges.fyi/challenge-wc/resources/test.txt";

fn main() {}

// int
// _DEFUN(iswspace,(c), wint_t c)
// {
// #ifdef _MB_CAPABLE
//   c = _jp2uc (c);
//   /* Based on Unicode 5.2.  Control chars 09-0D, plus all characters
//      from general category "Zs", which are not marked as decomposition
//      type "noBreak". */
//   return ((c >= 0x0009 && c <= 0x000d) || c == 0x0020 ||
// 	  c == 0x1680 || c == 0x180e ||
// 	  (c >= 0x2000 && c <= 0x2006) ||
// 	  (c >= 0x2008 && c <= 0x200a) ||
// 	  c == 0x2028 || c == 0x2029 ||
// 	  c == 0x205f || c == 0x3000);
// #else
//   return (c < 0x100 ? isspace (c) : 0);
// #endif /* _MB_CAPABLE */
// }
fn iswspace(c: u16) -> bool {
      (c >= 0x0009 && c <= 0x000d) || c == 0x0020 ||
	  c == 0x1680 || c == 0x180e ||
	  (c >= 0x2000 && c <= 0x2006) ||
	  (c >= 0x2008 && c <= 0x200a) ||
	  c == 0x2028 || c == 0x2029 ||
	  c == 0x205f || c == 0x3000
    // TODO: add !_MB_CAPABLE
}

struct Solution;

impl Solution {
    fn minus_c() -> Result<usize, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        File::open(TEST_FILE_PATH)?
            .read_to_end(&mut buf)?;
        Ok(buf.len().into())
    }

    fn minus_l() -> Result<usize, Box<dyn std::error::Error>>  {
        let mut buf = Vec::new();
        File::open(TEST_FILE_PATH)?
            .read_to_end(&mut buf)?;

        Ok(buf.iter().filter(|el| **el == b'\n').count())
    }

    // fn minus_m() -> Result<usize, Box<dyn std::error::Error>> {
    //     Ok(0)
    // }

    fn minus_w() -> Result<usize, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        File::open(TEST_FILE_PATH)?
            .read_to_end(&mut buf)?;

        Ok(String::from_utf8(buf.iter()
            .map(|el| if iswspace(*el as u16) { b' ' } else { *el })
            .collect::<Vec<u8>>())?
            .split(" ")
            .into_iter()
            .filter(|el| el.len() > 0)
            .count()
        )
    }
}


#[cfg(test)]
mod tests {
    use std::{process::{Command}};
    use super::*;

    fn get_system_wc(flag: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let out = Command::new("wc")
            .arg(flag)
            .arg(TEST_FILE_PATH)
            .output()?;

        let stdout: String = (*String::from_utf8(out.stdout)?
            .split(" ")
            .filter(|el| el.len() > 0)
            .collect::<Vec<&str>>()
            .get(0)
            .ok_or("err")?)
            .into();

        Ok(stdout.parse()?)
    }

    #[test]
    fn test_minus_c() {
        assert_eq!(get_system_wc("-c").unwrap(), Solution::minus_c().unwrap())
    }

    #[test]
    fn test_minus_l() {
        assert_eq!(get_system_wc("-l").unwrap(), Solution::minus_l().unwrap())
    }

    #[test]
    fn test_minus_w() {
        assert_eq!(get_system_wc("-w").unwrap(), Solution::minus_w().unwrap())
    }
}
