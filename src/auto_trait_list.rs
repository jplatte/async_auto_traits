use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

struct AutoTrait {
    ident: Ident,
    modifier: Option<Token![!]>,
}

impl Parse for AutoTrait {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let modifier = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { ident, modifier })
    }
}

pub struct AutoTraitList(Punctuated<AutoTrait, Token![+]>);

impl AutoTraitList {
    pub fn partition(self) -> (Vec<Ident>, Vec<Ident>) {
        let mut regular = Vec::new();
        let mut opt_out = Vec::new();

        for tr in self.0 {
            match tr.modifier {
                None => regular.push(tr.ident),
                Some(_) => opt_out.push(tr.ident),
            }
        }

        (regular, opt_out)
    }
}

impl Parse for AutoTraitList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Punctuated::parse_separated_nonempty(input).map(Self)
    }
}
