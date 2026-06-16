// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/errors/services/resolver.rs
//  Desc:       Error resolution service.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::errors::types::{BaseErrorType, ErrorInfo, ErrorCategory, LexErrorType, LexErrorInfo, ParseErrorInfo};
use crate::compiler::parser::errors::ParseErrorType;
use std::sync::OnceLock;

pub struct ErrorService;

impl ErrorService {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve(&self, category: ErrorCategory, error_type: BaseErrorType) -> ErrorInfo {
        let (message, suggestion, details) = self.get_message(&error_type);
        let severity = error_type.get_severity();

        ErrorInfo::new(category, error_type, message, suggestion, details, severity)
    }

    pub fn resolve_custom(&self, category: ErrorCategory, message: String, suggestion: Option<String>, details: Option<String>) -> ErrorInfo {
        ErrorInfo::custom(category, message, suggestion, details)
    }

    fn get_message(&self, error_type: &BaseErrorType) -> (String, Option<String>, Option<String>) {
        match error_type {
            BaseErrorType::Parser(pt) => self.get_parse_message(pt),

            BaseErrorType::InvalidFloat => (
                "Invalid float number format".to_string(),
                None,
                None
            ),
            BaseErrorType::InvalidInteger => (
                "Invalid integer format".to_string(),
                None,
                None
            ),
            BaseErrorType::InvalidHex => (
                "Invalid hexadecimal number format".to_string(),
                Some("Hexadecimal numbers must start with 0x and contain only 0-9, a-f, A-F characters.".to_string()),
                Some("Example: 0x1A2B, 0xFF, 0xdeadbeef.".to_string())
            ),
            BaseErrorType::InvalidBinary => (
                "Invalid binary number format".to_string(),
                Some("Binary numbers must start with 0b and contain only 0-1 characters.".to_string()),
                Some("Example: 0b1010, 0b11111111.".to_string())
            ),
            BaseErrorType::InvalidScientific => (
                "Invalid scientific notation format".to_string(),
                None,
                Some("The exponent part must be an integer.".to_string())
            ),
            BaseErrorType::NumberTooLarge => (
                "Number too large".to_string(),
                Some("Add 'n' suffix to create a BigInt for very large numbers.".to_string()),
                Some("48-bit integer range: -(2^47) to (2^47)-1.".to_string())
            ),
            BaseErrorType::HexNumberTooLarge => (
                "Hexadecimal number too large".to_string(),
                Some("Add 'n' suffix to create a BigInt for very large hexadecimal numbers.".to_string()),
                Some("48-bit integer range: -(2^47) to (2^47)-1.".to_string())
            ),
            BaseErrorType::BinaryNumberTooLarge => (
                "Binary number too large".to_string(),
                Some("Add 'n' suffix to create a BigInt for very large binary numbers.".to_string()),
                Some("48-bit integer range: -(2^47) to (2^47)-1.".to_string())
            ),
            
            BaseErrorType::UnexpectedChar => (
                "Unexpected character".to_string(),
                None,
                Some("Refer to the DinoCode specification for valid characters.".to_string())
            ),
            BaseErrorType::UnexpectedTokenAfterDot => (
                "Unexpected token after dot in this context".to_string(),
                Some("Specify a property or method name after the dot.".to_string()),
                Some("Example: object.property, object.method()".to_string())
            ),
            BaseErrorType::UnexpectedBlankAfterDot => (
                "Unexpected blank after dot in this context".to_string(),
                Some("Remove the space after the dot to access object properties or methods.".to_string()),
                Some("Example: object.property, object.method()".to_string())
            ),
            BaseErrorType::UnexpectedDollarCall => (
                "Unexpected $ symbol".to_string(),
                Some("Use the dollar call format: $(function_name arg1 arg2 ...) or remove the $ symbol.".to_string()),
                Some("The $ symbol is reserved for function calls and cannot be used as an identifier.".to_string())
            ),
            BaseErrorType::DollarCallWithSpace => (
                "Space after $ symbol".to_string(),
                Some("Remove the space after $ if you intended to make a dollar call like $(function_name arg1 arg2 ...)".to_string()),
                Some("The $ symbol is reserved for function calls and cannot be used as an identifier.".to_string())
            ),
            BaseErrorType::OperatorNotAllowed => (
                "Operator not allowed in this context".to_string(),
                Some("The operator's context indicates that it is unary, but it shouldn't be.".to_string()),
                None
            ),
            BaseErrorType::InvalidOperator => (
                "Invalid operator".to_string(),
                Some("Check the operator syntax and ensure it's supported".to_string()),
                None
            ),
            
            BaseErrorType::IncompleteRedirection => (
                "Literal redirection block is ambiguous".to_string(),
                Some("The expression and '>' operator must be on the same line".to_string()),
                Some("The expression is being mixed with the content of the following indented block.".to_string())
            ),
            BaseErrorType::UnexpectedSemicolon => (
                "Unexpected semicolon".to_string(),
                Some("Remove the semicolon.".to_string()),
                Some("Semicolons are used to separate statements.".to_string())
            ),
            BaseErrorType::ReservedKeywordAsIdentifier => (
                "Reserved keyword used as identifier".to_string(),
                Some("Use a different name that is not a reserved keyword".to_string()),
                Some("Reserved keywords include: if, while, return, etc.".to_string())
            ),
            BaseErrorType::Custom(msg) => (msg.clone(), None, None),
        }
    }

    fn get_parse_message(&self, error_type: &ParseErrorType) -> (String, Option<String>, Option<String>) {
        match error_type {
            ParseErrorType::ExpectedRightParen => ("Mismatched parenthesis, expected ')'".to_string(), Some("Ensure you close the parenthesis".to_string()), None),
            ParseErrorType::ExpectedLeftParen => ("Mismatched parenthesis, expected '('".to_string(), Some("Ensure you open the parenthesis".to_string()), None),
            ParseErrorType::ExpectedRightBrace => ("Mismatched brace, expected '}'".to_string(), Some("Ensure you close the brace".to_string()), None),
            ParseErrorType::ExpectedLeftBrace => ("Mismatched brace, expected '{'".to_string(), Some("Ensure you open the brace".to_string()), None),
            ParseErrorType::ExpectedRightBracket => ("Mismatched bracket, expected ']'".to_string(), Some("Ensure you close the bracket".to_string()), None),
            ParseErrorType::ExpectedLeftBracket => ("Mismatched bracket, expected '['".to_string(), Some("Ensure you open the bracket".to_string()), None),
            ParseErrorType::ExpectedStringTerminator => ("Mismatched string delimiter, expected string terminator".to_string(), None, None),
            ParseErrorType::ExpectedStringInitializer => ("Mismatched string delimiter, expected string initializer".to_string(), None, None),
            ParseErrorType::ExpectedRightBraceExpr => ("Mismatched interpolation brace, expected '}'".to_string(), Some("Ensure you close the interpolation expression".to_string()), None),
            ParseErrorType::ExpectedLeftBraceExpr => ("Mismatched interpolation brace, expected '${'".to_string(), Some("Ensure you open the interpolation expression".to_string()), None),
            ParseErrorType::ExpectedExpression(group) => (format!("Expected expression in {}", group), Some("Provide a valid expression".to_string()), Some("The group requires at least one expression".to_string())),
            ParseErrorType::MismatchedDelimiter(msg) => (msg.clone(), None, None),

            ParseErrorType::ExpectedIdentifier => ("Expected identifier".to_string(), Some("Provide a valid identifier name".to_string()), Some("Identifiers must start with a letter or underscore".to_string())),
            ParseErrorType::MismatchedParentheses => ("Mismatched parentheses".to_string(), Some("Check for missing or extra parentheses".to_string()), None),
            ParseErrorType::MismatchedBrackets => ("Mismatched brackets".to_string(), Some("Check for missing or extra brackets".to_string()), None),
            ParseErrorType::MismatchedBraces => ("Mismatched braces".to_string(), Some("Check for missing or extra braces".to_string()), None),
            ParseErrorType::FunctionNotFound => ("Function not found".to_string(), Some("Check if the function name is correct".to_string()), Some("Make sure the function is defined before use".to_string())),
            ParseErrorType::FunctionNotInScope => ("Function not found in scope".to_string(), Some("Check scope".to_string()), None),
            ParseErrorType::CannotAccessParentScopeFunction => ("Cannot access function from parent scope".to_string(), None, None),
            ParseErrorType::MultipleReturnValues => ("Multiple return values not supported".to_string(), None, None),
            ParseErrorType::ReturnOutsideFunction => ("Cannot return outside of a function".to_string(), None, None),
            ParseErrorType::BreakOutsideLoop => ("Cannot break outside of a loop".to_string(), Some("Use 'break' only inside 'while' or 'for' loops".to_string()), None),
            ParseErrorType::ContinueOutsideLoop => ("Cannot continue outside of a loop".to_string(), Some("Use 'continue' only inside 'while' or 'for' loops".to_string()), None),
            ParseErrorType::ExpectedClassName => ("Expected class name".to_string(), None, None),
            ParseErrorType::UnexpectedTokenInParameterList => ("Unexpected token in parameter list".to_string(), None, None),
            ParseErrorType::UnexpectedDollarCall => ("Unexpected $ symbol".to_string(), Some("Use the dollar call format: $(function_name arg1 arg2 ...) or remove the $ symbol.".to_string()), Some("The $ symbol is reserved for function calls and cannot be used as an identifier.".to_string())),
            ParseErrorType::ExpectedFunctionName => ("Expected function name after ':' symbol".to_string(), Some("Provide a valid function name".to_string()), Some("Function names must start with a letter or underscore".to_string())),
            ParseErrorType::InvalidAssignmentTarget => ("Invalid assignment target".to_string(), Some("Assignment target must be a variable or object property".to_string()), Some("You can only assign to variables, object properties, or use compound assignment operators".to_string())),
            ParseErrorType::InvalidTypeIndexForAsOperator => ("Invalid type index for 'as' operator".to_string(), Some("The 'as' operator expects a valid type identifier".to_string()), Some("Example: expression as String, expression as Number".to_string())),
            ParseErrorType::InvalidTypeIndexForIsOperator => ("Invalid type index for 'is' operator".to_string(), Some("The 'is' operator expects a valid type identifier".to_string()), Some("Example: expression is String, expression is Number".to_string())),
            ParseErrorType::UnexpectedIsOperator => ("Unexpected 'is' operator".to_string(), Some("Type check must follow an expression".to_string()), Some("Place the expression before 'is': value is Type".to_string())),
            ParseErrorType::UnexpectedInOperator => ("Unexpected 'in' operator".to_string(), Some("The 'in' operator must be used in a for loop".to_string()), Some("Format: for variable in iterable".to_string())),
            ParseErrorType::NativePropertyAssignment => ("Cannot assign to native property".to_string(), Some("Native properties are read-only".to_string()), Some("Use regular object properties with '.' instead of native properties with '@' if you need to assign values".to_string())),
            ParseErrorType::UnsupportedBackdotOperator => ("Unsupported '@' operator".to_string(), Some("The '@' symbol is only used for accessing native properties and methods".to_string()), Some("Use '@' for access: object@property or object@method(), not as a binary operator".to_string())),
            ParseErrorType::ExpectedTypeIdentifierAfterIs => ("Expected type identifier after 'is'".to_string(), Some("Provide a valid type name".to_string()), Some("Example: value is String, value is Number".to_string())),
            ParseErrorType::ExpectedTypeIdentifierAfterAs => ("Expected type identifier after 'as'".to_string(), Some("Provide a valid type name".to_string()), Some("Example: value as String, value as Number".to_string())),
            ParseErrorType::UnexpectedOperatorInDollarCall => ("Unexpected operator in dollar call".to_string(), Some("You can only call functions or methods with dollar syntax".to_string()), Some("Format: $functionName or $object.method()".to_string())),
            ParseErrorType::FunctionNotFoundInScope => ("Function not found in scope".to_string(), Some("Check if the function name is correct and accessible".to_string()), Some("Functions can only be accessed from current scope or global scope".to_string())),
            ParseErrorType::UndefinedVariable { name, suggestion } => (format!("Undefined variable: {}", name), suggestion.as_ref().map(|s| format!("Did you mean '{}'?", s)), Some("Make sure the variable is defined before use".to_string())),
            ParseErrorType::UnknownType { name, suggestion } => (format!("Type '{}' is not supported", name), suggestion.as_ref().map(|s| format!("Did you mean '{}'?", s)), Some("Check if the type name is correct".to_string())),
            ParseErrorType::MultipleMainFunction => ("Multiple main functions found".to_string(), Some("Only one 'main' function is allowed".to_string()), Some("Remove extra main functions or rename them".to_string())),
            ParseErrorType::InvalidUnaryOperator => ("Invalid unary operator".to_string(), Some("Check if the operator is supported as unary".to_string()), Some("Supported unary operators: -, !".to_string())),
            ParseErrorType::InvalidBinaryOperator => ("Invalid binary operator".to_string(), Some("Check if the operator is supported as binary".to_string()), Some("Supported binary operators: +, -, *, /, //, %, **, ==, !=, >, <, >=, <=, and, or, .".to_string())),
            ParseErrorType::PrefixIncrementDecrementNotSupported => ("Prefix increment/decrement operators are not supported".to_string(), Some("Use postfix increment/decrement (i++, i--) instead".to_string()), Some("Postfix increment/decrement is syntactic sugar for i += 1".to_string())),
            ParseErrorType::MatchCorrespondenceError { expected_values, actual_values } => ("Match correspondence error".to_string(), Some(format!("expected {} values for comparison, got {}", expected_values, actual_values)), Some("Ensure the number of expected values matches the actual values provided".to_string())),
            ParseErrorType::EmptyMatchComparison => ("Empty match comparison".to_string(), Some("Match expression requires at least one value to compare".to_string()), Some("Example: is 'val1' 'val2' '...'".to_string())),
            ParseErrorType::ExpectedIndentedBlock(token_type) => ("Expected indented block".to_string(), Some(format!("'{}' requires an indented block", format!("{:?}", token_type).to_lowercase())), Some("Add proper indentation after the statement".to_string())),
            
            ParseErrorType::Custom(msg) => (msg.clone(), None, None),
        }
    }

}

static GLOBAL_SERVICE: OnceLock<ErrorService> = OnceLock::new();

pub fn get_global_service() -> &'static ErrorService {
    GLOBAL_SERVICE.get_or_init(ErrorService::new)
}

pub fn resolve_error(category: ErrorCategory, error_type: BaseErrorType) -> ErrorInfo {
    get_global_service().resolve(category, error_type)
}

pub fn resolve_custom_error(category: ErrorCategory, message: String, suggestion: Option<String>, details: Option<String>) -> ErrorInfo {
    get_global_service().resolve_custom(category, message, suggestion, details)
}

pub fn resolve_lex_error(error_type: LexErrorType) -> LexErrorInfo {
    resolve_error(ErrorCategory::Lexer, error_type)
}

pub fn resolve_parse_error(error_type: ParseErrorType) -> ParseErrorInfo {
    resolve_error(ErrorCategory::Parser, BaseErrorType::Parser(error_type))
}

pub fn resolve_custom_lex_error(message: String, suggestion: Option<String>, details: Option<String>) -> LexErrorInfo {
    resolve_custom_error(ErrorCategory::Lexer, message, suggestion, details)
}


