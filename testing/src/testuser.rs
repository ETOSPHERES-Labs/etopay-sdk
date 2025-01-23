use chrono::{TimeZone, Utc};
use fake::{
    faker::{
        chrono::en::DateTimeBetween,
        internet::en::Username,
        name::{en::LastName, raw::FirstName},
    },
    locales::EN,
    Fake,
};
use lazy_static::lazy_static;

const PIN: &str = "12345";
const PASSWORD: &str = "StrongP@55w0rd";
pub const MNEMONIC_DEFAULT: &str = "aware mirror sadness razor hurdle bus scout crisp close life science spy shell fine loop govern country strategy city soldier select diet brain return";
pub const MNEMONIC_ALICE: &str = "gas way upon large hollow reason smart visual crazy boring rack deputy culture faith urge faint pulse smoke zebra broccoli same tail capable catch";
pub const MNEMONIC_HANS34: &str = "direct minute suggest salt fade grid drink brother genuine regular pen desert renew cliff lumber simple element army goose trouble machine begin humor measure";
pub const MNEMONIC_HANS48: &str = "enemy attend neither digital dizzy soap lecture note novel unusual swamp nuclear favorite license state perfect rebel guilt excess absorb twenty various hollow drum";

lazy_static! {
    /// Default test user satoshi verified everywhere
    pub static ref USER_SATOSHI: TestUser = create_test_user("satoshi", "Satoshi", "Nakamoto", "satoshi@gmail.com", MNEMONIC_DEFAULT);

    /// Verified users via Postident KYC
    pub static ref USER_ALICE: TestUser = create_test_user("alice", "Alice", "Wonderland", "alice@eto.viviswap.com", MNEMONIC_ALICE);
    pub static ref USER_HANS34: TestUser = create_test_user("hans34", "Hans34", "Sama", "hans34.maier34@gmx.com", MNEMONIC_HANS34);
    pub static ref USER_HANS48: TestUser = create_test_user("hans48", "Hans48", "Sama", "hans48.maier34@gmx.com", MNEMONIC_HANS48);

    // user BOB not KYC verified!
    // pub static ref USER_BOB: TestUser = create_test_user("bob", "Bob", "Swagger", "bob@eto.viviswap.com", MNEMONIC_DEFAULT);

    // user VIVI not KYC verified!
    // pub static ref USER_VIVI: TestUser = create_test_user("vivi", "Vivi", "Swap", "vivi@eto.viviswap.com", MNEMONIC_DEFAULT);

    /// Test user for archive purposes
    pub static ref USER_ARCHIVEME: TestUser = create_test_user("archiveme", "", "", "noop@eto.viviswap.com", MNEMONIC_DEFAULT);
}

fn create_test_user(username: &str, first_name: &str, last_name: &str, email: &str, mnemonic: &str) -> TestUser {
    TestUser {
        username: username.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        email: email.to_string(),
        mnemonic: mnemonic.to_string(),
        ..Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct TestUser {
    pub username: String,
    pub password: String,
    pub pin: String,
    pub mnemonic: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub iban: String,
}

impl Default for TestUser {
    fn default() -> Self {
        let first_name: String = FirstName(EN).fake();
        let last_name: String = LastName().fake();
        let username: String = Username().fake();
        let email = format!("{username}@eto.testuser.com");
        let date_of_birth = generate_date_of_birth();
        let iban = generate_iban();

        Self {
            username,
            password: PASSWORD.to_string(),
            pin: PIN.to_string(),
            mnemonic: MNEMONIC_DEFAULT.to_string(),
            first_name,
            last_name,
            email,
            date_of_birth,
            iban,
        }
    }
}

pub fn generate_date_of_birth() -> String {
    let start: chrono::DateTime<Utc> = Utc.timestamp_micros(0).unwrap();
    let end = Utc.with_ymd_and_hms(2004, 1, 1, 0, 0, 0).unwrap();
    let date: chrono::DateTime<Utc> = DateTimeBetween(start, end).fake();
    date.format("%Y-%m-%d").to_string()
}

pub fn generate_iban() -> String {
    let country_code = "DE";
    let bank_code = "50010517";
    let account_number = (1223456789..9992345678).fake::<u64>().to_string();
    // Convert letters to numbers (A = 10, B = 11, ..., Z = 35)
    let mut numeric_country_code = String::new();
    for ch in country_code.chars() {
        numeric_country_code.push_str(&(ch as u32 - 'A' as u32 + 10).to_string());
    }

    // Concatenate bank code and account number
    let mut bban = String::new();
    bban.push_str(bank_code);
    bban.push_str(&account_number);

    // Append the numeric country code and "00" (for the checksum)
    let mut check_string = String::new();
    check_string.push_str(&bban);
    check_string.push_str(&numeric_country_code);
    check_string.push_str("00");

    // Convert to integer and calculate the checksum
    let check_int: u128 = check_string.parse().unwrap();
    let checksum = 98 - (check_int % 97);

    // Format the IBAN
    format!("{}{:02}{}{}", country_code, checksum, bank_code, account_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_test_user() {
        let test_user = TestUser::default();
        println!("Test user: {:#?}", test_user);
    }

    #[test]
    fn should_create_a_valid_iban() {
        // DE91100000000123456789
        let iban = generate_iban();
        println!("IBAN: {}", iban);
    }
}
