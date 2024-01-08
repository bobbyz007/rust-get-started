use proc_macro::{TokenStream};
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, ItemFn, parenthesized, parse2, parse_macro_input, Token, token, Type};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

/// The procedural macros is more powerful as they can arbitrarily modify their input,
/// and produce any output desired as long as its within the bounds of the language syntax.
///

// function-like procedural macro
#[proc_macro]
pub fn define_struct_by_name(input: TokenStream) -> TokenStream {
    let name = input.to_string();
    let output = format!("#[derive(Debug)] struct {} {{ data: i32 }}", name);

    output.parse().unwrap()
}

struct FuncSig {
    fn_name: Ident,
    args: Vec<Type>,
    return_type: Type,
}
struct FuncSyntax {
    _fn_token: Token!(fn),
    fn_name: Ident,
    _paren_token: token::Paren,
    paren_fields: Punctuated<Type, Token![,]>,
    _arrow_token: Token!(->),
    return_type: Type,
}
impl Parse for FuncSig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            panic!("Write full function signature.");
        }

        let content;
        // 解析函数语法
        let syntax = FuncSyntax {
            _fn_token: input.parse().unwrap(),
            fn_name: input.parse().unwrap(),
            _paren_token: parenthesized!(content in input),
            paren_fields: content.parse_terminated(Type::parse, Token![,]).unwrap(),
            _arrow_token: input.parse().unwrap(),
            return_type: input.parse().unwrap(),
        };
        Ok(FuncSig {
            fn_name: syntax.fn_name,
            args: syntax.paren_fields.into_iter().collect(),
            return_type: syntax.return_type,
        })
    }
}

#[proc_macro]
pub fn make_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let signature = syn::parse_macro_input!(input as FuncSig);
    let fn_name = signature.fn_name;
    let args = signature.args;
    let return_type = signature.return_type;

    if let 1 = args.len()  {
        let arg = &args[0];
        let tokens = quote!{
            fn #fn_name(arg: #arg) -> #return_type {
                let ret: #return_type = arg * 2;
                println!("input {} * 2 = {}", arg, ret);
                ret
            }
        };
        tokens.into()
    } else {
        panic!("Invalid input");
    }
}

// attribute proc macro用于添加或修改代码结构中的元数据信息。可以被应用于诸如结构体、函数、模块、枚举等各种 Rust 语言元素上。
// _input: 属性本身，如 #[attr]的attr
// annotated_item: 属性标注的对象
#[proc_macro_attribute]
pub fn log_func_info(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    //  将输入函数解析为ItemFn
    let mut func = parse_macro_input!(annotated_item as ItemFn);
    let func_name = &func.sig.ident;
    let func_block = &func.block;
    let output = quote!({
        println!("fun {} starts", stringify!(#func_name));
        let log_result = { #func_block };
        println!("fun {} ends", stringify!(#func_name));
        log_result
    });
    func.block = parse2(output).unwrap();
    quote!(#func).into()
}

// derive procedural macro派生宏仅能应用于结构体或枚举上
#[proc_macro_derive(HelloMacroName)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput); // 解析input为 DeriveInput类型
    let target_ident = ast.ident;  // 获取原始类名
    let builder_ident = format_ident!("{}Builder", target_ident.to_string()); // 拼接builder类名

    // 处理结构体
    if let Data::Struct(r) = ast.data {
        let fields = r.fields;
        // builder属性声明， 添加Option
        let builder_fields = map_fields(&fields, &mut |(ident, ty)| {
            quote!(#ident: Option<#ty>,) // 注意要添加逗号分隔
        });
        // 为builder增加set函数
        let builder_set_fields = map_fields(&fields, &mut |(ident, ty)| {
            quote!(
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                }
           )
        });
        // 获取builder的属性值
        let builder_lets = map_fields(&fields, &mut |(ident, _)| {
            quote!(
                let #ident = self.#ident.ok_or(format!(
                    "field {:?}  not set yet", stringify!(#ident),
                ))?;
            )
        });
        // 初始化时的默认值
        let builder_fields_values = map_fields(&fields, &mut |(ident, _)| {
            quote!(
                #ident,
            )
        });
        quote!(
            impl #target_ident {
                pub fn builder() -> #builder_ident {
                    #builder_ident::default()
                }
            }

            #[derive(Default)]
            pub struct #builder_ident {
                #builder_fields
            }

            //实现builder
            impl #builder_ident {
                #builder_set_fields
                // 构建target对象
                pub fn build(self) -> Result<#target_ident, String> {
                    #builder_lets
                    Ok(#target_ident{ #builder_fields_values })
                }
            }
        ).into()
    } else {
        // 不支持非struct类型
        quote!().into()
    }
}

fn map_fields<F>(fields: &Fields, mapper:&mut F) -> TokenStream2
    where
        F: FnMut((&Option<proc_macro2::Ident> ,  &Type)) -> TokenStream2,
{
    let fs = fields.iter().map(|field| mapper((&field.ident ,&field.ty)) );
    let stream2 = TokenStream2::from_iter(fs);
    stream2
}