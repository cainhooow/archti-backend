#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentKind {
    Cpf,
    Cnpj,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    value: String,
    kind: DocumentKind,
}

impl Document {
    pub fn parse(raw: impl AsRef<str>) -> Result<Self, String> {
        let value: String = raw
            .as_ref()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();

        let kind = match value.len() {
            11 if is_valid_cpf(&value) => DocumentKind::Cpf,
            14 if is_valid_cnpj(&value) => DocumentKind::Cnpj,
            _ => return Err(String::from("Document must be a valid CPF or CNPJ")),
        };

        Ok(Self { value, kind })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn kind(&self) -> &DocumentKind {
        &self.kind
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn is_valid_cpf(value: &str) -> bool {
    if value.len() != 11 || all_digits_equal(value) {
        return false;
    }

    let digits = to_digits(value);

    let first_digit = calculate_check_digit(&digits[..9], &[10, 9, 8, 7, 6, 5, 4, 3, 2]);
    let second_digit = calculate_check_digit(&digits[..10], &[11, 10, 9, 8, 7, 6, 5, 4, 3, 2]);

    digits[9] == first_digit && digits[10] == second_digit
}

fn is_valid_cnpj(value: &str) -> bool {
    if value.len() != 14 || all_digits_equal(value) {
        return false;
    }

    let digits = to_digits(value);

    let first_digit = calculate_check_digit(&digits[..12], &[5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
    let second_digit =
        calculate_check_digit(&digits[..13], &[6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);

    digits[12] == first_digit && digits[13] == second_digit
}

fn calculate_check_digit(digits: &[u32], weights: &[u32]) -> u32 {
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&digit, &weight)| digit * weight)
        .sum();

    let remainder = sum % 11;

    if remainder < 2 { 0 } else { 11 - remainder }
}

fn to_digits(value: &str) -> Vec<u32> {
    value.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn all_digits_equal(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    chars.all(|ch| ch == first)
}
