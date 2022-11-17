use quote::ToTokens;
use syn::{parse2, Field, Ident, Path, TypePath, Visibility};

#[derive(Debug)]
pub struct FieldData {
    pub f_ident: Ident,
    pub f_name: String,
    pub f_is_pub: bool,
    pub f_type_path: Path,
    pub f_type_ident: Ident,
}

impl From<Field> for FieldData {
    fn from(f: Field) -> FieldData {
        let f_ident = f.ident.unwrap();
        let f_name = f_ident.to_string();
        let f_is_pub = matches!(f.vis, Visibility::Public(_));
        let f_type_path = parse2::<TypePath>(f.ty.to_token_stream()).unwrap().path;
        let f_type_ident = f_type_path.segments.last().unwrap().ident.clone();
        // let f_type_args = f_type_path
        //     .segments
        //     .first()
        //     .map(|seg| seg.arguments.clone())
        //     .unwrap();

        FieldData {
            f_ident,
            f_name,
            f_is_pub,
            f_type_path,
            f_type_ident,
        }
    }
}
