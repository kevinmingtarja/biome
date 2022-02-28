use crate::{
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{TsPropertySignatureClassMember, TsPropertySignatureClassMemberFields};

impl ToFormatElement for TsPropertySignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPropertySignatureClassMemberFields {
            access_modifier,
            abstract_token,
            readonly_token,
            name,
            property_annotation,
            semicolon_token,
        } = self.as_fields();

        let access_modifier = access_modifier
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let abstract_token = abstract_token.format(formatter)?;
        let readonly_token = readonly_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let property_annotation = property_annotation.format_or_empty(formatter)?;

        let semicolon_token = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            access_modifier,
            abstract_token,
            space_token(),
            readonly_token,
            name.format(formatter)?,
            property_annotation,
            semicolon_token
        ])
    }
}
