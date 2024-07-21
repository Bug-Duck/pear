use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn handler(_ : TokenStream, input : TokenStream) -> TokenStream {
  let item_fn = parse_macro_input!(input as ItemFn);
  let origin = item_fn.sig.ident.clone();
  let name = Ident::new(
    &format!("{}_{}", item_fn.sig.ident.to_string(), "service"),
    Span::call_site(),
  );
  let first = item_fn.sig.inputs.iter().next().unwrap();
  let ty = match first {
    FnArg::Typed(paty) => paty.ty.clone(),
    _ => {
      panic!("FUCK YOU");
    }
  };

  let output = quote! {
    #item_fn

    pub struct #name;
    impl Handler for #name {
      fn match_event(&self, event : &SwarmEvent<PearBehaviourEvent>) -> bool {
        #ty::try_match(event)
      }
      fn handle(&self, event : SwarmEvent<PearBehaviourEvent>, state : PearContext) -> futures::future::BoxFuture<()> {
        use futures::{FutureExt};
        async move {
          #origin(#ty::extract_from(event).unwrap(), state).await
        }.boxed()
      }
    }
  };

  TokenStream::from(output)
}
