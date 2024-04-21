pub struct Arn {
    pub arn: Box<str>,
    pub arn_type: ArnType,
}
pub enum ArnType {
    Function,
    Layer,
}

pub fn parse_arn(arn: &str) -> Result<Arn, String> {
    let parts: Vec<&str> = arn.split(':').collect();
    if arn.starts_with("arn:") && parts.len() >= 7 {
        Ok(Arn { arn: arn.into(), arn_type: function_or_layer_arn(arn).expect("ARN must be a Lambda function or Layer") })
    } else {
        Err("ARN must start with 'arn:', and have 8 parts including a version tag".to_string())
    }
}

pub fn function_or_layer_arn(arn: &str) -> Result<ArnType, String> {
    let parts: Vec<&str> = arn.split(':').collect();
    match parts[5] {
        "function" => Ok(ArnType::Function),
        "layer" => Ok(ArnType::Layer),
        _ => Err("ARN must be a Lambda function or Layer".to_string())
    }
}

pub fn get_region(arn: &str) -> String {
    let parts: Vec<&str> = arn.split(':').collect();
    parts[3].to_string()
}
