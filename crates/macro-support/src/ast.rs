use crate::error::Errors;
use crate::field::Field;
use syn;

// fn struct_from_ast<'a>(cx: &Ctxt, data: &'a syn::`Field, attrs: Option<&attr::Variant>) -> (Style, Vec<Field<'a>>) {
//     match *data {
//         syn::Field::Struct(ref fields) => (Style::Struct, fields_from_ast(cx, fields, attrs)),
//         syn::Field::Tuple(ref fields) if fields.len() == 1 => {
//             (Style::Newtype, fields_from_ast(cx, fields, attrs))
//         }
//         syn::Field::Tuple(ref fields) => (Style::Tuple, fields_from_ast(cx, fields, attrs)),
//         syn::Field::Unit => (Style::Unit, Vec::new()),
//     }
// }

pub fn fields_from_ast<'a>(errors: &Errors, fields: Vec<syn::Field>) -> Vec<Field> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field::from_ast(errors, i, field))
        .collect()
}
