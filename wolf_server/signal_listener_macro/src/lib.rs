extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
mod utils;
use crate::utils::to_snake_case;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Token, Type};

#[proc_macro]
pub fn define_signal_listener(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as SignalListenerInput);
    let name_as_string = input.name.to_string();
    let snake_case_name = to_snake_case(&name_as_string);
    let trait_name = Ident::new(
        &format!("{}SignalListener", name_as_string),
        Span::call_site(),
    );
    let sender_trait_name = Ident::new(
        &format!("{}SignalSender", name_as_string),
        Span::call_site(),
    );
    let holder_trait_name = Ident::new(
        &format!("{}SignalHolder", name_as_string),
        Span::call_site(),
    );
    let send_signal_func_name = Ident::new(
        &format!("send_{}_signal", snake_case_name),
        Span::call_site(),
    );
    let send_signal_to_func_name = Ident::new(
        &format!("send_{}_signal_to", snake_case_name),
        Span::call_site(),
    );
    let receive_signal_func_name = Ident::new(
        &format!("receive_{}_signal", snake_case_name),
        Span::call_site(),
    );
    let remove_listener_func_name = Ident::new(
        &format!("remove_{}_signal_listener", snake_case_name),
        Span::call_site(),
    );
    let add_listener_func_name = Ident::new(
        &format!("add_{}_signal_listener", snake_case_name),
        Span::call_site(),
    );
    let listeners_type_name = Ident::new(
        &format!("{}ListenersType", name_as_string),
        Span::call_site(),
    );
    let game_arg = if input.game_ref.mutability.is_some() {
        quote! {
            game: &mut Game
        }
    } else {
        quote! {
            game: &Game
        }
    };
    let args = input.args;
    let arg_names_only: Vec<Ident> = args.iter().map(|arg| arg.arg_name.clone()).collect();
    let init_result = input.return_type.clone().map(|return_type| {
        let ty = return_type.ty;
        quote! {
            let mut collected_result: Option<#ty>  = None;
        }
    });
    let collect_result = input.return_type.clone().map(|_| quote!(let result =));
    let combine_result = input.return_type.clone().map(|_| {
        quote! {
            if let Some(current_collected_result) = collected_result{
                collected_result = Some(current_collected_result.combine_result(result));
            }
            else{
                collected_result = Some(result);
            }
        }
    });
    let return_clause = input.return_type.clone().map(|_| {
        quote! {
            return collected_result
        }
    });
    let return_type_with_arrow_optional = input.return_type.clone().map(|t| {
        let ty = t.ty;
        quote!(-> Option<#ty>)
    });
    let return_type_with_arrow = input.return_type.clone().map(|t| {
        let ty = t.ty;
        quote!(-> #ty)
    });
    let early_signal_send_return = if input.return_type.is_some() {
        quote!(return None)
    } else {
        quote!(return)
    };
    let output = quote! {
        type #listeners_type_name = id::IdMap<ComponentId, Box<#trait_name>>;
        pub trait #trait_name{
            fn #receive_signal_func_name(&self, #game_arg, owner: GameObjectId, #(#args),*)#return_type_with_arrow;
            fn get_listener_id(&self) -> ComponentId;
            fn clone_box(&self) -> Box<dyn #trait_name>;
        }
        pub trait #sender_trait_name{
            fn #send_signal_func_name(&self, #game_arg, #(#args),*)#return_type_with_arrow_optional;
            fn #send_signal_to_func_name(&self, #game_arg, listener_id: ComponentId, #(#args),*)#return_type_with_arrow_optional;
        }
        impl #sender_trait_name for GameObjectId{
            fn #send_signal_func_name(&self, #game_arg, #(#args),*)#return_type_with_arrow_optional{
                let listeners = {
                    if let Some(this) = game.game_objects.get(*self){
                        this.listeners.get::<#listeners_type_name>()
                            .map(|listeners|listeners
                                 .iter()
                                 .map(|(_key, item)|item.clone_box())
                                 .collect::<Vec<Box<#trait_name>>>()
                            )
                    }
                    else{
                        #early_signal_send_return
                    }
                };
                #init_result
                if let Some(listeners) = listeners{
                    for listener in listeners.iter(){
                        #collect_result listener.#receive_signal_func_name(
                            game,
                            self.clone(),
                            #(#arg_names_only),*
                        );
                        #combine_result
                    }
                }
                #return_clause
            }
            fn #send_signal_to_func_name(&self, #game_arg, listener_id: ComponentId, #(#args),*)#return_type_with_arrow_optional{
                if let Some(this) = game.game_objects.get(*self){
                    if let Some(listeners) = this.listeners.get::<#listeners_type_name>() {
                        if let Some(listener) = listeners.get(listener_id) {
                            let cloned_listener = listener.clone_box();
                            let collected_result = Some(cloned_listener.#receive_signal_func_name(
                                game,
                                self.clone(),
                                #(#arg_names_only),*
                            ));
                            #return_clause
                        }
                    }
                }
                #early_signal_send_return
            }
        }
        pub trait #holder_trait_name{
            fn #add_listener_func_name<T: #trait_name + 'static>(&mut self, listener: T);
            fn #remove_listener_func_name(&mut self, id: ComponentId);
        }
        impl #holder_trait_name for GameObject{
            fn #add_listener_func_name<T: #trait_name + 'static>(&mut self, listener_unboxed: T){
                let listener: Box<dyn #trait_name> = Box::new(listener_unboxed);
                let listeners = self.listeners
                    .entry::<#listeners_type_name>()
                    .or_insert(id::IdMap::new());
                listeners.insert(listener.get_listener_id(), listener);
            }
            fn #remove_listener_func_name(&mut self, id: ComponentId) {
                let listeners_entry = self.listeners.entry::<#listeners_type_name>();
                if let anymap::Entry::Occupied(mut listeners_occupied) = listeners_entry {
                    let listeners = listeners_occupied.get_mut();
                    listeners.remove(id);
                    if(listeners.is_empty()) {
                        listeners_occupied.remove();
                    }
                }
            }
        }
        impl GameObjectId{
            pub fn #add_listener_func_name<T: #trait_name + 'static>(&self, game: &mut Game, listener: T){
                let game_object = game.game_objects.get_mut(*self).unwrap();
                game_object.#add_listener_func_name(listener);
            }
            pub fn #remove_listener_func_name(&self, game: &mut Game, id: ComponentId) {
                if let Some(game_object) = game.game_objects.get_mut(*self){
                    game_object.#remove_listener_func_name(id);
                }
            }
        }
    };
    output.into()
}

#[derive(Debug, Clone)]
struct Argument {
    arg_name: Ident,
    arg_type: Type,
}
impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let arg_name = self.arg_name.clone();
        let arg_type = self.arg_type.clone();
        let output = quote!(
        #arg_name : #arg_type
                       );

        tokens.extend(output);
    }
}
impl Parse for Argument {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg_name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let arg_type = input.parse::<Type>()?;
        Ok(Argument { arg_name, arg_type })
    }
}
#[derive(Clone)]
struct ReturnType {
    ty: Type,
}
struct ArgumentAndOptionalReturnType {
    argument: Argument,
    return_type: Option<ReturnType>,
}
impl Parse for ArgumentAndOptionalReturnType {
    fn parse(input: ParseStream) -> Result<Self> {
        let argument = input.parse::<Argument>()?;
        let return_type = input.parse::<ReturnType>().ok();
        Ok(ArgumentAndOptionalReturnType {
            argument,
            return_type,
        })
    }
}
impl Parse for ReturnType {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![->]>()?;
        let ty = input.parse::<Type>()?;
        Ok(ReturnType { ty })
    }
}
mod kw {
    syn::custom_keyword!(Game);
}
struct GameRef {
    mutability: Option<Token![mut]>,
}
impl Parse for GameRef {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![&]>()?;
        let mutability = input.parse::<Token![mut]>().ok();
        input.parse::<kw::Game>()?;
        Ok(GameRef { mutability })
    }
}
struct SignalListenerInput {
    name: Ident,
    game_ref: GameRef,
    args: Vec<Argument>,
    return_type: Option<ReturnType>,
}
impl Parse for SignalListenerInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        let _ignore_comma = input.parse::<Token![,]>();
        let game_ref = input.parse::<GameRef>()?;
        let early_return = input.parse::<ReturnType>().ok();
        if early_return.is_some() {
            return Ok(SignalListenerInput {
                name,
                game_ref,
                args: Vec::new(),
                return_type: early_return,
            });
        }
        let _ignore_comma = input.parse::<Token![,]>();
        let args_with_separators =
            Punctuated::<ArgumentAndOptionalReturnType, Token![,]>::parse_terminated(input)?;
        let mut return_type = None;
        let args: Vec<Argument> = args_with_separators
            .pairs()
            .map(|pair| {
                let value = pair.into_value();
                if value.return_type.is_some() {
                    return_type = value.return_type.clone()
                }
                value.argument.clone()
            })
            .collect();
        Ok(SignalListenerInput {
            name,
            game_ref,
            args,
            return_type,
        })
    }
}
