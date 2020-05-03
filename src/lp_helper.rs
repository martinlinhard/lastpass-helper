use crate::rofi_helper::execute_rofi;
use std::process::Command;
pub struct LPHelper {
    acc: Option<String>,
    acc_string: String,
}

impl LPHelper {
    pub fn new() -> Self {
        let output = Command::new("lpass").arg("ls").output().unwrap();
        let output: String = String::from_utf8(output.stdout).unwrap();
        let results: &str = output.trim();

        let mut accounts = vec![];
        results.split("\n").for_each(|acc| {
            let remainder = acc.split("/").nth(1).unwrap();
            if remainder.starts_with(" ") {
                return;
            }
            let remainder = &remainder[0..remainder.find('[').unwrap()];
            accounts.push(remainder.trim().to_owned());
        });

        let acc_string = accounts.join("\n");

        Self {
            acc: None,
            acc_string,
        }
    }
    pub fn get_account(&mut self) {
        self.acc = Some(execute_rofi(&self.acc_string));
    }

    pub fn copy_password(&self) {
        if let Some(ref acc) = self.acc {
            Command::new("lpass")
                .args(&["show", "-c", "--password", acc.trim()])
                .spawn()
                .unwrap();
        }
    }

    pub fn copy_username(&self) {
        if let Some(ref acc) = self.acc {
            Command::new("lpass")
                .args(&["show", "-c", "--username", acc.trim()])
                .spawn()
                .unwrap();
        }
    }
}
